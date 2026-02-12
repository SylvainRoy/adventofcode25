use std::fs;
use num_bigint::BigUint;
use std::str::FromStr;

struct Range {
    start: BigUint,
    end: BigUint,
}

impl Range {
    fn from_str(s: &str) -> Range {
        let parts: Vec<&str> = s.split('-').collect();
        let start = BigUint::from_str(parts[0]).unwrap();
        let end = BigUint::from_str(parts[1]).unwrap();
        Range { start, end }
    }

    fn invalid_ids_part1(&self) -> Vec<BigUint> {
        let mut invalid_ids = vec![];
        
        // Determine the maximum number of digits we need to check
        let end_str = self.end.to_string();
        let max_digits = end_str.len();
        
        // Check all possible demi-lengths (1 digit, 2 digits, etc.)
        for demi_len in 1..=max_digits {
            // Generate all possible demis of this length
            let min_demi = BigUint::from(10u32).pow((demi_len - 1) as u32);
            let max_demi = BigUint::from(10u32).pow(demi_len as u32) - BigUint::from(1u32);
            
            let mut demi = min_demi.clone();
            loop {
                if demi > max_demi {
                    break;
                }
                
                // Create the invalid ID by concatenating demi with itself
                let demi_str = demi.to_string();
                let invalid_id_str = format!("{}{}", demi_str, demi_str);
                let invalid_id = BigUint::from_str(&invalid_id_str).unwrap();
                
                // Check if this invalid ID is within our range
                if invalid_id >= self.start && invalid_id <= self.end {
                    invalid_ids.push(invalid_id.clone());
                }
                
                // If the invalid ID is already larger than our end, 
                // no need to check larger demis of this length
                if invalid_id > self.end {
                    break;
                }
                
                demi += BigUint::from(1u32);
            }
        }
        
        invalid_ids.sort();
        invalid_ids
    }

    fn invalid_ids_part2(&self) -> Vec<BigUint> {
        let mut invalid_ids = vec![];
        
        // Determine the maximum number of digits we need to check
        let end_str = self.end.to_string();
        let max_digits = end_str.len();
        
        // Check all possible demi-lengths (1 digit, 2 digits, etc.)
        'outer: for demi_len in 1..=max_digits {
            // Generate all possible demis of this length
            let min_demi = BigUint::from(10u32).pow((demi_len - 1) as u32);
            let max_demi = BigUint::from(10u32).pow(demi_len as u32) - BigUint::from(1u32);
            
            let mut demi = min_demi.clone();
            loop {
                if demi > max_demi {
                    break;
                }
                
                // Create invalid IDs by repeating demi multiple times
                let mut repeat_count = 2;
                loop {
                    // Create invalid IDs
                    let invalid_id_str_multi = demi.to_string().repeat(repeat_count);
                    let invalid_id_multi = BigUint::from_str(&invalid_id_str_multi).unwrap();
                    
                    if invalid_id_multi >= self.start && invalid_id_multi <= self.end {
                        invalid_ids.push(invalid_id_multi.clone());
                    } else if invalid_id_multi > self.end {
                        // If invalid_id is greater than self.end and repeat_count == 2, 
                        // exit both loops
                        if repeat_count == 2 {
                            break 'outer;
                        }
                        break;
                    }
                    
                    repeat_count += 1;
                }
                
                demi += BigUint::from(1u32);
            }
        }
        
        invalid_ids.sort();
        invalid_ids.dedup();
        invalid_ids
    }
}

impl ToString for Range {
    fn to_string(&self) -> String {
        format!("{}-{}", self.start, self.end)
    }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input file");
    let ranges = input
        .trim()
        .split(',')
        .map(Range::from_str)
        .collect::<Vec<Range>>();

    let sum_invalid_ids: BigUint = ranges.iter()
        .map(|r| r.invalid_ids_part1().iter().fold(BigUint::from(0u32), |acc, x| acc + x))
        .fold(BigUint::from(0u32), |acc, x| acc + x);
    println!("Part One: {}", sum_invalid_ids);

    let sum_invalid_ids: BigUint = ranges.iter()
        .map(|r| r.invalid_ids_part2().iter().fold(BigUint::from(0u32), |acc, x| acc + x))
        .fold(BigUint::from(0u32), |acc, x| acc + x);
    println!("Part Two: {}", sum_invalid_ids); // too high: 46742684357
}
