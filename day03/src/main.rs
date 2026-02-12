use std::fs;
use num_bigint::BigUint;


fn main() {
    let data = fs::read_to_string("data/input.txt").expect("Failed to read the file");
    let banks = data
        .lines()
        .map(|line| line
            .chars()
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let part1 : u32 = banks
        .iter()
        .map(|bank| maxjoltage1(bank))
        .sum();
    println!("Part 1: {}", part1); // 17359

    let part2 : BigUint = banks
        .iter()
        .map(|bank| maxjoltage2(bank, 12, Vec::new())
            .iter()
            .fold(BigUint::from(0u32), |acc, v| BigUint::from(10u32) * acc + BigUint::from(u32::from(*v) - 48u32)))
        .fold(BigUint::from(0u32), |acc, v| acc + v);
    println!("Part 2: {}", part2); // 172787336861064
}

fn maxjoltage1(bank: &Vec<char>) -> u32 {
    let mut i = 0;
    let mut j = 1;
    for n in 1..bank.len() {
        if n < (bank.len() - 1) && bank[n] > bank[i] {
            i = n;
            j = n + 1;
        } else if bank[n] > bank[j] {
            j = n;
        }
    }
    // println!("{:?} = {:?}", bank.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(""), 10 * bank[i] + bank[j]);
    return 10 * (u32::from(bank[i]) - 48) + (u32::from(bank[j]) - 48);
}

fn maxjoltage2(bank: &[char], digits: usize, mut res: Vec<char>) -> Vec<char> {
    if digits == 0 {
        return res;
    }
    let (i_max, c_max) = bank[..(bank.len() - digits + 1)]
        .iter()
        .enumerate()
        .reduce(|(i_max, c_max), (i, c)| if c > c_max { (i, c) } else { (i_max, c_max) })
        .unwrap();
    res.push(*c_max);
    return maxjoltage2(&bank[i_max + 1..], digits - 1, res);
}
