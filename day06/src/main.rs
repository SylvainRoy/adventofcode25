use std::fs;

fn main() {
    let content = fs::read_to_string("data/input.txt").expect("Failed to read the file");
    let lines = content.lines().collect::<Vec<&str>>();

    let cells = lines[..lines.len() - 1]
        .iter()
        .map(|line| { line
            .split_ascii_whitespace()
            .map(|item| item.parse::<u64>().expect("Failed to parse a number"))
            .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let ops = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .map(|item| item.chars().next().expect("Failed to parse an operator"))
        .collect::<Vec<char>>();

    let mut result = 0;
    for i in 0..cells[0].len() {
        if ops[i] == '+' {
            result += cells.iter().map(|row| row[i]).sum::<u64>();
        } else {
            result += cells.iter().map(|row| row[i]).product::<u64>();
        }
    }
    println!("Part One: {}", result);

    let rows = lines[..lines.len() - 1]
        .iter()
        .map(|line| line
            .chars()
            .chain(std::iter::once(' '))
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();


    let ops = lines[lines.len() - 1]
        .chars()
        .chain(std::iter::once(' '))
        .collect::<Vec<char>>();

    let mut operation = ' ';
    let mut result = 0;
    let mut numbers = vec![];
    for x in 0..rows[0].len() {
        if ops[x] != ' ' {
            operation = ops[x];
        }
        let digits = rows
            .iter()
            .fold(
                String::new(), 
                |acc, row| if row[x].to_digit(10).is_some() {
                    acc + row[x].to_string().as_str()
                } else { 
                    acc 
                });
        if let Ok(num) = digits.parse::<u64>() {
            numbers.push(num);
        } else {
            if operation == '+' {
                result += numbers.iter().sum::<u64>();
            } else {
                result += numbers.iter().product::<u64>();
            }
            numbers.clear();
            operation = ops[x];
        }
    }
    println!("Part Two: {:?}", result); // 11744693538946
}
