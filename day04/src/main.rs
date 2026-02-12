use std::fs;

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<u32>>,
}

impl Grid {
    fn from_string(s: &str) -> Self {
        let cells: Vec<Vec<u32>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == '@' { 1 } else { 0 })
                    .collect()
            })
            .collect();
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };
        Grid { width, height, cells }
    }

    fn get(&self, x: i32, y: i32) -> u32 {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.cells[x as usize][y as usize]
        } else {
            0
        }
    }

    fn neighbours(&self, x: i32, y: i32) -> u32 {
        let mut result = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                result += self.get(x + dx, y + dy);
            }
        }
        result
    }

    fn remove(&mut self) -> u32 {
        let mut counter = 0;
        for i in 0..self.height as i32 {
            for j in 0..self.width as i32 {
                if self.get(i, j) == 1 && self.neighbours(i, j) < 4 {
                    self.cells[i as usize][j as usize] = 0;
                    counter += 1;
                }
            }
        }
        return counter;
    }
}

fn main() {
    let content = fs::read_to_string("data/input.txt")
        .expect("Failed to read the file");
    let mut grid = Grid::from_string(&content);

    let mut counter = 0;
    for i in 0..grid.height as i32 {
        for j in 0..grid.width as i32 {
            if grid.get(i, j) != 1 { continue; }
            if grid.neighbours(i, j) >= 4 { continue; }
            counter += 1;
        }
    }
    println!("Part One: {}", counter);

    let mut counter = 0;
    loop {
        let removed = grid.remove();
        if removed == 0 { break; }
        counter += removed;
    }
    println!("Part Two: {}", counter);
}
