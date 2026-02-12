use std::fs;

fn main() {
    // Read the input file
    let content = fs::read_to_string("data/input.txt").expect("Failed to read input file");
    let layers = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let size = layers[0].len();

    // Part One
    let mut beams = vec![false; size];
    let mut split_count = 0;
    for layer in &layers {
        let mut next_beams = vec![false; size];
        for (i, &ch) in layer.iter().enumerate() {
            match ch {
                'S' => next_beams[i] = true,
                '^' => {
                    if !beams[i] { continue; }
                    if i > 0 { next_beams[i - 1] = true }
                    if i + 1 < size { next_beams[i + 1] = true }
                    split_count += 1;
                },
                _ => next_beams[i] |= beams[i],
            }
        }
        beams = next_beams;
    }
    println!("Part One: {:?}", split_count); // 1590

    // Part Two
    let mut beams : Vec<u64>= vec![0; size];
    for layer in &layers {
        let mut next_beams : Vec<u64> = vec![0; size];
        for (i, &ch) in layer.iter().enumerate() {
            match ch {
                'S' => next_beams[i] += 1,
                '^' => {
                    if beams[i] == 0 { continue; }
                    if i > 0 { next_beams[i - 1] += beams[i] }
                    if i + 1 < size { next_beams[i + 1] += beams[i] }
                },
                _ => next_beams[i] += beams[i],
            }
        }
        beams = next_beams;
    }
    println!("Part Two: {:?}", beams.iter().sum::<u64>());
}
