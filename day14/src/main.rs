use std::{cmp::Ordering, fs};

const WIDTH: usize = 101;
const HEIGHT: usize = 103;
const MIDDLE_X: usize = WIDTH / 2;
const MIDDLE_Y: usize = HEIGHT / 2;
const SIMULATION_STEPS_PART1: usize = 100;

#[derive(Debug)]
pub struct Robot {
    position: (usize, usize),
    velocity: (i32, i32),
}

impl Robot {
    pub fn from(line: &str) -> Result<Self, ()> {
        let splits = line
            .split(" ")
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();
        if splits.len() != 2 {
            eprintln!("Failed to parse robot as <position>, <velocity>.");
            return Err(());
        }
        let (position, velocity) = (&splits[0], &splits[1]);
        let position = position
            .strip_prefix("p=")
            .expect("position should start with p=");
        let velocity = velocity
            .strip_prefix("v=")
            .expect("position should start with v=");
        let position = parse_xy(position)?;
        if position.0 < 0 || position.1 < 0 {
            eprintln!("Position should be non-negative.");
            return Err(());
        }
        let position = (position.0 as usize, position.1 as usize);
        let velocity = parse_xy(velocity)?;
        Ok(Self { position, velocity })
    }

    pub fn update(&mut self) {
        let (x, y) = self.position;
        let (dx, dy) = self.velocity;
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        self.position = (proper_mod(nx, WIDTH), proper_mod(ny, HEIGHT));
    }

    pub fn quadrant(&self) -> Option<usize> {
        match (
            self.position.0.cmp(&MIDDLE_X),
            self.position.1.cmp(&MIDDLE_Y),
        ) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => None,
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
        }
    }
}

fn proper_mod(a: i32, b: usize) -> usize {
    (a % (b as i32) + (b as i32)) as usize % b
}

fn parse_xy(split: &str) -> Result<(i32, i32), ()> {
    let splits = split
        .split(",")
        .map(|l| {
            l.trim()
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '-')
                .collect::<String>()
        })
        .map(|s| s.parse())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| eprintln!("Failed to parse int: {e}"))?;
    if splits.len() != 2 {
        eprintln!("Invalid X, Y section.");
        return Err(());
    }
    Ok((splits[0], splits[1]))
}

fn parse_input() -> Result<Vec<Robot>, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    raw.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(Robot::from)
        .collect()
}

fn sim_robots(robots: &mut [Robot]) {
    for r in robots {
        r.update();
    }
}

fn quadrant_counts_product(robots: &[Robot]) -> u32 {
    let mut quadrants: [u32; 4] = [0; 4];
    for q in robots.iter().filter_map(|r| r.quadrant()) {
        quadrants[q] += 1;
    }
    quadrants.iter().product()
}

fn display_board(board: [u32; HEIGHT * WIDTH]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cur = y * WIDTH + x;
            if board[cur] == 0 {
                print!(".");
            } else {
                print!("{}", board[cur]);
            }
        }
        println!();
    }
}

fn build_board(robots: &[Robot]) -> [u32; HEIGHT * WIDTH] {
    let mut board: [u32; HEIGHT * WIDTH] = [0; HEIGHT * WIDTH];
    for r in robots {
        let cur = r.position.1 * WIDTH + r.position.0;
        board[cur] += 1;
    }
    board
}

fn main() {
    let mut robots = parse_input().unwrap();
    let mut snd = None;
    let mut fst = None;

    let mut step = 0;
    loop {
        if step == SIMULATION_STEPS_PART1 {
            fst = Some(quadrant_counts_product(&robots));
        }
        if build_board(&robots).iter().all(|r| *r <= 1) {
            snd = Some(step);
        }
        if fst.is_some() && snd.is_some() {
            break;
        }
        sim_robots(&mut robots);
        step += 1;
    }

    display_board(build_board(&robots));
    println!("Day 14, part 1: {}", fst.unwrap());
    println!("Day 14, part 2: {}", snd.unwrap());
}
