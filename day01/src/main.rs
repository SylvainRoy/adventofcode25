use std::fs;

struct Rotation {
    direction: char,
    degrees: i32,
}

impl Rotation {
    fn from_str(s: &str) -> Option<Rotation> {
        let direction = s.chars().next()?;
        let degrees = s[1..].parse().ok()?;
        Some(Rotation { direction, degrees })
    }
    fn as_degrees(&self) -> i32 {
        match self.direction {
            'L' => -self.degrees,
            'R' => self.degrees,
            _ => 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").expect("Failed to read input file");
    let rotations = input
        .lines()
        .map(Rotation::from_str)
        .collect::<Option<Vec<Rotation>>>()
        .expect("Failed to parse rotations");

    let mut angle = 50;
    let mut num_zeros_part_one = 0;
    let mut num_zeros_part_two = 0;
    for rotation in rotations {

        // Part 2
        let div = rotation.as_degrees() / 100;
        let rem = rotation.as_degrees() % 100;
        num_zeros_part_two += div.abs();
        let to_angle = angle + rem;
        if angle != 0 && (to_angle <= 0 || 99 < to_angle) {
            num_zeros_part_two += 1;
        }

        // Part 1
        angle = (angle + rotation.as_degrees()).rem_euclid(100);
        if angle == 0 {
            num_zeros_part_one += 1;
        }
    }
    println!("Part One: {}", num_zeros_part_one);
    println!("Part Two: {}", num_zeros_part_two); // not 9317
}
