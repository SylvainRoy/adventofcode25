use std::fs;

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn from_string(s: &str) -> Self {
        let sep = s.find('-').expect("Invalid range format");
        let start = s[..sep].parse::<u64>().expect("Invalid start of range");
        let end = s[sep + 1..].parse::<u64>().expect("Invalid end of range");
        Range { start, end }
    }   

    fn contains(&self, id: u64) -> bool {
        self.start <= id && id <= self.end
    }
}

fn main() {
    let content = fs::read_to_string("data/input.txt").expect("Failed to read the file");
    let lines = content.lines();

    let mut ranges = lines
        .clone()
        .take_while(|line| !line.is_empty())
        .map(|line| Range::from_string(line))
        .collect::<Vec<Range>>();

    let ids = lines
        .clone()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse::<u64>().expect("Invalid ID format"))
        .collect::<Vec<u64>>();

    let mut fresh = 0;
    for id in ids {
        for range in &ranges {
            if range.contains(id) {
                fresh += 1;
                break;
            }
        }
    }
    println!("Part One: {}", fresh); // 733

    ranges.sort_by_key(|range| range.start);
    let mut fresh = 0;
    let mut i = 0;
    for range in ranges {
        if i < range.start {
            i = range.start;
        }
        if i > range.end {
            continue;
        }
        fresh += range.end - i + 1;
        i = range.end + 1;
    }   
    println!("Part Two: {}", fresh); // 345821388687084
}
