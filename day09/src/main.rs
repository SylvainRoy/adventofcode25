use std::fs;

struct Tile {
    pos: (usize, usize),
}

impl Tile {
    fn from_str(s: &str) -> Tile {
        let mut pos = s.split(",");
        Tile {
            pos: (
                pos.next().unwrap().parse().unwrap(),
                pos.next().unwrap().parse().unwrap(),
            ),
        }
    }
}

fn main() {
    let content = fs::read_to_string("./data/input.txt").unwrap();
    let tiles: Vec<Tile> = content.lines().map(Tile::from_str).collect();

    //
    // Part One
    //
    let mut maxarea = 0;
    for i in 0..tiles.len() {
        for j in (i + 1).min(tiles.len())..tiles.len() {
            let dx = tiles[i].pos.0.abs_diff(tiles[j].pos.0) + 1;
            let dy = tiles[i].pos.1.abs_diff(tiles[j].pos.1) + 1;
            let area = dx * dy;
            if area > maxarea {
                maxarea = area;
            }
        }
    }
    println!("Part One: {}", maxarea); // 4763040296

    //
    // Part Two
    //

    // Create a sparse grid representation of the floor
    let mut xs = tiles.iter().map(|t| t.pos.0).collect::<Vec<usize>>();
    xs.sort();
    xs.dedup();
    let mut xxs = Vec::new();
    xxs.push(0);
    for x in xs {
        xxs.push(x);
        xxs.push(x + 1);
    }
    xxs.dedup();
    let x_to_idx = xxs
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect::<std::collections::HashMap<usize, usize>>();

    let mut ys = tiles.iter().map(|t| t.pos.1).collect::<Vec<usize>>();
    ys.sort();
    ys.dedup();
    let mut yys = Vec::new();
    yys.push(0);
    for y in ys {
        yys.push(y);
        yys.push(y + 1);
    }
    yys.dedup();
    let y_to_idx = yys
        .iter()
        .enumerate()
        .map(|(i, y)| (*y, i))
        .collect::<std::collections::HashMap<usize, usize>>();

    let mut grid = vec![vec!['.'; yys.len()]; xxs.len()];

    // Draw the connections on the floor
    for (tstart, tend) in tiles.iter().zip(tiles.iter().chain(tiles.iter()).skip(1)) {
        if tstart.pos.0 == tend.pos.0 {
            let ystart = y_to_idx[&tstart.pos.1.min(tend.pos.1)];
            let yend = y_to_idx[&tstart.pos.1.max(tend.pos.1)];
            for y in ystart..=yend {
                grid[x_to_idx[&tstart.pos.0]][y] = '#';
            }
        }
        if tstart.pos.1 == tend.pos.1 {
            let xstart = x_to_idx[&tstart.pos.0.min(tend.pos.0)];
            let xend = x_to_idx[&tstart.pos.0.max(tend.pos.0)];
            for x in xstart..=xend {
                grid[x][y_to_idx[&tstart.pos.1]] = '#';
            }
        }
    }

    // Fill the internal area (in fact, empty the external one)
    empty_outside(0, 0, &mut grid);
    // draw_floor(&grid);

    // Find the biggest, completely filled rectangle
    let mut maxarea = 0;
    for i in 0..tiles.len() {
        for j in (i + 1).min(tiles.len())..tiles.len() {
            let dx = tiles[i].pos.0.abs_diff(tiles[j].pos.0) + 1;
            let dy = tiles[i].pos.1.abs_diff(tiles[j].pos.1) + 1;
            let area = dx * dy;
            if area > maxarea
                && rect_filled(
                    x_to_idx[&tiles[i].pos.0],
                    y_to_idx[&tiles[i].pos.1],
                    x_to_idx[&tiles[j].pos.0],
                    y_to_idx[&tiles[j].pos.1],
                    &grid,
                )
            {
                maxarea = area;
            }
        }
    }
    println!("Part Two: {}", maxarea); // 1396494456
}

fn draw_floor(floor: &Vec<Vec<char>>) {
    for y in 0..floor[0].len() {
        for x in 0..floor.len() {
            print!("{}", floor[x][y]);
        }
        println!();
    }
}

fn empty_outside(i: i32, j: i32, grid: &mut Vec<Vec<char>>) {
    if i < 0
        || i >= grid.len() as i32
        || j < 0
        || j >= grid[0].len() as i32
        || grid[i as usize][j as usize] == '#'
        || grid[i as usize][j as usize] == ' '
    {
        return;
    }
    grid[i as usize][j as usize] = ' ';
    empty_outside(i + 1, j, grid);
    empty_outside(i - 1, j, grid);
    empty_outside(i, j + 1, grid);
    empty_outside(i, j - 1, grid);
}

fn rect_filled(x1: usize, y1: usize, x2: usize, y2: usize, grid: &Vec<Vec<char>>) -> bool {
    for x in x1.min(x2)..=x1.max(x2) {
        for y in y1.min(y2)..=y1.max(y2) {
            if grid[x][y] == ' ' {
                return false;
            }
        }
    }
    true
}
