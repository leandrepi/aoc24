use std::{fmt, fs};

const BARRIER_CHAR: u8 = "#".as_bytes()[0];
const START_CHAR: u8 = "S".as_bytes()[0];
const END_CHAR: u8 = "E".as_bytes()[0];
// dir_y, dir_x: East West North South
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const MIN_CHEAT_GAIN: usize = 100;

#[derive(Debug, Clone)]
pub struct CharArray {
    contents: Vec<u8>,
    width: usize,
    height: usize,
}

impl<Idx> std::ops::Index<Idx> for CharArray
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.contents[index]
    }
}

impl<Idx> std::ops::IndexMut<Idx> for CharArray
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.contents[index]
    }
}

impl CharArray {
    fn from(raw: &str) -> Result<Self, ()> {
        let mut lines = raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let first = lines
            .next()
            .expect("Should have at least a non-empty line.");
        let mut contents = first.bytes().collect::<Vec<u8>>();
        let width = first.len();
        let mut height = 1;
        for line in lines {
            if line.len() != width {
                eprintln!(
                    "Invalid char array, line {} differs in width ({} vs {}).",
                    height + 1,
                    line.len(),
                    width
                );
                return Err(());
            }

            contents.extend(line.bytes());
            height += 1;
        }
        Ok(Self {
            contents,
            width,
            height,
        })
    }

    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }
}

impl fmt::Display for CharArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, &c) in self.contents.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

fn find_start(map: &CharArray) -> (usize, usize) {
    let mut start = None;
    let mut end = None;
    for (c, &x) in map.contents.iter().enumerate() {
        if let (Some(s), Some(e)) = (start, end) {
            return (s, e);
        }
        if x == START_CHAR {
            start = Some(c);
        }
        if x == END_CHAR {
            end = Some(c);
        }
    }
    unreachable!()
}

fn walk_map(map: &CharArray, start: usize, end: usize) -> Vec<usize> {
    let mut prev = Vec::with_capacity(map.contents.len());
    let mut visited = vec![false; map.contents.len()];
    let mut c = start;
    prev.push(c);
    visited[start] = true;
    while c != end {
        let x = c % map.width;
        let y = c / map.width;
        for (dir_y, dir_x) in DIRECTIONS.iter() {
            let ny = y as i32 + dir_y;
            let nx = x as i32 + dir_x;
            if !map.is_valid(nx, ny) {
                continue;
            }
            let nc = ny as usize * map.width + nx as usize;
            if map[nc] == BARRIER_CHAR || visited[nc] {
                continue;
            }
            c = nc;
            prev.push(nc);
            visited[nc] = true;
            break;
        }
    }
    prev
}

fn cheat_values(map: &CharArray, path: &[usize], allowed_steps: usize) -> usize {
    let mut res = 0;
    let mut scores = vec![0; map.contents.len()];
    for (i, &p) in path.iter().enumerate() {
        scores[p] = i;
    }

    for (p_idx, c) in path.iter().enumerate().take(path.len() - MIN_CHEAT_GAIN) {
        let y = c / map.width;
        let x = c % map.width;

        let min_y = if y <= allowed_steps {
            0
        } else {
            y - allowed_steps
        };
        let max_y = if y + allowed_steps >= map.height {
            map.height
        } else {
            y + allowed_steps + 1
        };

        for ny in min_y..max_y {
            let ysteps = (ny as i32 - y as i32).unsigned_abs() as usize;
            let xsteps = allowed_steps - ysteps;
            let min_x = if x <= xsteps { 0 } else { x - xsteps };
            let max_x = if x + xsteps >= map.width {
                map.width
            } else {
                x + xsteps + 1
            };
            let mut nc = ny * map.width + min_x;
            for nx in min_x..max_x {
                let steps = ysteps + (nx as i32 - x as i32).unsigned_abs() as usize;
                if scores[nc] >= MIN_CHEAT_GAIN + p_idx + steps {
                    res += 1;
                }
                nc += 1;
            }
        }
    }
    res
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let map = CharArray::from(&raw).unwrap();
    let (start, end) = find_start(&map);
    let path = walk_map(&map, start, end);
    let fst = cheat_values(&map, &path, 2);
    println!("Day 20, part 1: {}", fst);
    let snd = cheat_values(&map, &path, 20);
    println!("Day 20, part 2: {}", snd);
}
