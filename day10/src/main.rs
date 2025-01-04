use std::{collections::HashSet, fs};

#[derive(Debug)]
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

impl CharArray {
    fn from(raw: &str) -> Self {
        let mut lines = raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let first = lines
            .next()
            .expect("Should have at least a non-empty line.");
        let mut contents = first
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).expect("ascii") as u8)
            .collect::<Vec<u8>>();
        let width = first.len();
        let mut height = 1;
        for line in lines {
            contents.extend(
                line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| c.to_digit(10).expect("ascii") as u8),
            );
            height += 1;
        }
        Self {
            contents,
            width,
            height,
        }
    }

    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }
}

fn count_trailheads_from_pos(map: &CharArray, y: usize, x: usize, acc: u8) -> HashSet<usize> {
    if acc == 9 {
        let mut result = HashSet::new();
        let _ = result.insert(y * map.width + x);
        return result;
    }
    let mut res = HashSet::new();
    for (dir_x, dir_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = x as i32 + dir_x;
        let ny = y as i32 + dir_y;
        if map.is_valid(nx, ny) && map[ny as usize * map.width + nx as usize] == acc + 1 {
            for c in count_trailheads_from_pos(map, ny as usize, nx as usize, acc + 1) {
                let _ = res.insert(c);
            }
        }
    }
    res
}

fn count_ratings_from_pos(map: &CharArray, y: usize, x: usize, acc: u8) -> u32 {
    if acc == 9 {
        return 1;
    }
    let mut res = 0;
    for (dir_x, dir_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = x as i32 + dir_x;
        let ny = y as i32 + dir_y;
        if map.is_valid(nx, ny) && map[ny as usize * map.width + nx as usize] == acc + 1 {
            res += count_ratings_from_pos(map, ny as usize, nx as usize, acc + 1);
        }
    }
    res
}

fn count_trailheads(map: &CharArray) -> usize {
    let mut res = 0;
    for (idx, _) in map.contents.iter().enumerate().filter(|(_, &c)| c == 0) {
        let acc = 0;
        res += count_trailheads_from_pos(map, idx / map.width, idx % map.width, acc).len();
    }
    res
}

fn count_ratings(map: &CharArray) -> u32 {
    let mut res = 0;
    for (idx, _) in map.contents.iter().enumerate().filter(|(_, &c)| c == 0) {
        let acc = 0;
        res += count_ratings_from_pos(map, idx / map.width, idx % map.width, acc);
    }
    res
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let map = CharArray::from(&raw);
    let fst = count_trailheads(&map);
    println!("Day 10, part 1: {fst}");
    let snd = count_ratings(&map);
    println!("Day 10, part 2: {snd}");
}
