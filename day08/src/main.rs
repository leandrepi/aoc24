use std::{
    collections::{HashMap, HashSet},
    fmt, fs,
};

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

impl<Idx> std::ops::IndexMut<Idx> for CharArray
where
    Idx: std::slice::SliceIndex<[u8]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.contents[index]
    }
}

impl CharArray {
    fn from(raw: &str) -> Self {
        let mut lines = raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let first = lines
            .next()
            .expect("Should have at least a non-empty line.");
        let mut contents = first.bytes().collect::<Vec<u8>>();
        let width = first.len();
        let mut height = 1;
        for line in lines {
            contents.extend(line.bytes());
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

fn gather_antennas(map: &CharArray) -> HashMap<u8, Vec<usize>> {
    let dot = ".".bytes().next().expect("ascii char");
    let mut antennas = HashMap::new();
    for (idx, c) in map.contents.iter().enumerate().filter(|(_, &c)| c != dot) {
        antennas
            .entry(*c)
            .and_modify(|v: &mut Vec<usize>| v.push(idx))
            .or_insert(vec![idx]);
    }
    antennas
}

fn antinodes_from_pair(map: &CharArray, fst: usize, snd: usize, shallow: bool) -> Vec<usize> {
    let mut res = vec![];
    let width = map.width;
    let fst_y = fst / width;
    let fst_x = fst % width;
    let snd_y = snd / width;
    let snd_x = snd % width;
    let dx = fst_x as i32 - snd_x as i32;
    let dy = fst_y as i32 - snd_y as i32;
    let mut ax = fst_x as i32 + dx;
    let mut ay = fst_y as i32 + dy;
    let mut steps = 1;

    for step_dir in [1, -1] {
        // my kingdom for a do while
        loop {
            if map.is_valid(ax, ay) {
                res.push((ay as usize) * width + ax as usize);
            } else {
                break;
            }
            if shallow {
                break;
            }
            ax += step_dir * dx;
            ay += step_dir * dy;
            steps += 1;
        }
        ax -= (steps + 2) * dx;
        ay -= (steps + 2) * dy;
    }
    res
}

fn count_antinodes(map: &CharArray, shallow: bool) -> usize {
    let antennas = gather_antennas(map);
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (idx, &fst) in positions[..(positions.len() - 1)].iter().enumerate() {
            for &snd in positions[(idx + 1)..].iter() {
                let mut pair_anti = antinodes_from_pair(map, fst, snd, shallow);
                if !shallow && !pair_anti.is_empty() {
                    pair_anti.push(fst);
                    pair_anti.push(snd);
                }
                for antinode in pair_anti {
                    let _ = antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let map = CharArray::from(&raw);
    let fst = count_antinodes(&map, true);
    println!("Day 8, part 1: {fst}");
    let snd = count_antinodes(&map, false);
    println!("Day 8, part 2: {snd}");
}
