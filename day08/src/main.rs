use std::fs;

#[derive(Debug)]
struct Box {
    pub pos: Vec<i64>
}

impl Box {
    fn from_str(s: &str) -> Box {
        let dims: Vec<i64> = s
            .split(',')
            .filter_map(|part| part.parse::<i64>().ok())
            .collect();
        Box {
            pos: dims,
        }
    }

    fn dist(&self, other: &Box) -> i64 {
        self.pos.iter().zip(other.pos.iter())
            .map(|(a, b)| (a - b).pow(2))
            .sum()
    }
}

fn main() {
    let content = fs::read_to_string("data/input.txt").expect("Failed to read the file");
    let boxes = content
        .lines()
        .map(Box::from_str)
        .collect::<Vec<Box>>();

    let mut distances : Vec<(i64, usize, usize)>= Vec::with_capacity(boxes.len().pow(2) / 2);
    for i in 0..boxes.len() {
        for j in 0..i {
            let dist = boxes[i].dist(&boxes[j]) as i64;
            distances.push((dist, i, j));
        }
    }
    distances.sort_by(|a, b| a.0.cmp(&b.0));

    let mut circuits: Vec<std::collections::HashSet<usize>> = Vec::new();
    let mut index = 0;
    for _icxn in 0..1000 {
        if let Some((_dist, a, b)) = distances.get(index) {
            connect(*a, *b, &mut circuits, &boxes);
            index += 1;
            // println!("Circuit sizes: {:?} with {} cxns\n", circuits.iter().map(|set| set.len()).collect::<Vec<usize>>(), _icxn);
        }
    }

    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|set| set.len()).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    // println!("Circuit sizes: {:?}", circuit_sizes);
    println!("Part One: {:?}", circuit_sizes[..3].iter().product::<usize>()); // 102816

    while circuits.len() > 1 || circuits[0].len() < boxes.len() {
        if let Some((_dist, a, b)) = distances.get(index) {
            connect(*a, *b, &mut circuits, &boxes);
            index += 1;
            // println!("Circuit sizes: {:?}", circuits.iter().map(|set| set.len()).collect::<Vec<usize>>());
        }
    }

    let (_, a, b) = distances[index-1];
    println!("Part Two: {:?}", boxes[a].pos[0] * boxes[b].pos[0]); // 100011612
}


fn connect(a: usize, b: usize, circuits: &mut Vec<std::collections::HashSet<usize>>, boxes: &Vec<Box>) {
    let iset_a = circuits.iter().position(|set| set.contains(&a));
    let iset_b = circuits.iter().position(|set| set.contains(&b));
    // println!("{:?} in {:?} - {:?} in {:?})", boxes[a], iset_a, boxes[b], iset_b);

    match (iset_a, iset_b) {
        (Some(ia), Some(ib)) => {
            if ia != ib {
                let set_b = circuits.remove(ib.max(ia));
                circuits[ia.min(ib)].extend(set_b);
            }
        },
        (Some(ia), None) => {
            circuits[ia].insert(b);
        },
        (None, Some(ib)) => {
            circuits[ib].insert(a);
        },
        (None, None) => {
            let mut new_set = std::collections::HashSet::new();
            new_set.insert(a);
            new_set.insert(b);
            circuits.push(new_set);
        },
    }
}