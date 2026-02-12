use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Machine {
    indicators: Vec<i32>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i32>,
}

impl Machine {
    fn from(input: &str) -> Self {
        let tokens = input.split(" ").collect::<Vec<&str>>();
        let indicators: Vec<i32> = tokens[0]
            .chars()
            .filter_map(|c| match c {
                '.' => Some(0),
                '#' => Some(1),
                _ => None,
            })
            .collect();
        let buttons = tokens[1..tokens.len() - 1]
            .iter()
            .map(|token| {
                token[1..token.len() - 1]
                    .split(",")
                    .map(|s| s.parse().expect("Failed to parse button"))
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let joltages = tokens[tokens.len() - 1][1..tokens[tokens.len() - 1].len() - 1]
            .split(",")
            .map(|s| s.parse::<i32>().expect("Failed to parse joltage"))
            .collect::<Vec<i32>>();
        Self {
            indicators,
            buttons,
            joltages,
        }
    }

    fn check_indicators(&self, state: &[i32]) -> bool {
        self.indicators
            .iter()
            .zip(state.iter())
            .all(|(a, b)| a == b)
    }

    fn solve_part_one(&self) -> usize {
        for num_buttons in 1..self.indicators.len() {
            for combination in (0..self.buttons.len()).combinations(num_buttons) {
                let mut state = vec![0; self.indicators.len()];
                for &button_index in &combination {
                    for &wiring in self.buttons[button_index].iter() {
                        state[wiring] = 1 - state[wiring];
                    }
                }
                if self.check_indicators(&state) {
                    return num_buttons;
                }
            }
        }
        panic!("No solution found");
    }

    fn solve_part_two(&self, state: &[i32]) -> usize {
        if state.iter().all(|x| *x == 0) {
            return 0;
        }

        let odds = state
            .iter()
            .map(|x| if x.rem_euclid(2) == 1 { 1 } else { 0 })
            .collect::<Vec<i32>>();

        if odds.iter().all(|x| *x == 0) {
            let res = self.solve_part_two(&state.iter().map(|x| x / 2).collect::<Vec<i32>>());
            if res == usize::MAX {
                return usize::MAX;
            }
            return 2 * res;
        }

        //
        // Find all combinations of buttons that cancels the odd part of the joltages.
        //

        let mut combinations: Vec<Vec<usize>> = vec![];
        let mut deltas = vec![0; self.indicators.len()];

        // For each combination of buttons
        for num_buttons in 1..=self.indicators.len() {
            for combination in (0..self.buttons.len()).combinations(num_buttons) {
                // build the resulting delta on the state
                deltas.fill(0);
                for &button_index in &combination {
                    for &wiring in self.buttons[button_index].iter() {
                        deltas[wiring] = 1 - deltas[wiring];
                    }
                }
                // check if this delta cancels the odd part of the joltages
                let mut ismatch = true;
                for (odd, delta) in odds.iter().zip(deltas.iter()) {
                    if odd != delta {
                        ismatch = false;
                        break;
                    }
                }
                if ismatch {
                    combinations.push(combination);
                }
            }
        }

        //
        // For each combination, remove the odd part and recurse on the even part.
        //

        let mut shortest = usize::MAX;
        for combination in combinations {
            // Compute the new state
            let mut new_state: Vec<i32> = state.to_vec();
            for &button_index in &combination {
                for &wiring in self.buttons[button_index].iter() {
                    new_state[wiring] -= 1;
                }
            }
            let mut nonnegative = true;
            for i in 0..new_state.len() {
                if new_state[i] < 0 {
                    nonnegative = false;
                    break;
                }
                new_state[i] /= 2;
            }
            if !nonnegative {
                continue;
            }

            // Recurse
            let res = self.solve_part_two(&new_state);
            if res == usize::MAX {
                continue;
            }
            let res = combination.len() + 2 * res;
            shortest = shortest.min(res);
        }
        shortest
    }
}

fn main() {
    let input = fs::read_to_string("data/input.txt").unwrap();
    let machines: Vec<Machine> = input.lines().map(|line| Machine::from(line)).collect();

    if true {
        let partone = machines
            .iter()
            .map(|machine| machine.solve_part_one())
            .sum::<usize>();
        println!("Part One: {}", partone); // 385
    }

    if true {
        let res = machines
            .iter()
            .map(|machine| machine.solve_part_two(&machine.joltages))
            .sum::<usize>();
        println!("Part Two: {}", res); // 16757
    }
}
