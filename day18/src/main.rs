use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt, fs,
};

const BARRIER_CHAR: u8 = "#".as_bytes()[0];
const DOT_CHAR: u8 = ".".as_bytes()[0];
const GRID_SIZE: usize = 71;
const PART_ONE_BARRIERS: usize = 1024;
const START_POS: usize = 0;
const END_POS: usize = GRID_SIZE * GRID_SIZE - 1;
// dir_y, dir_x: East West North South
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

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

fn parse_barriers() -> Result<Vec<usize>, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    let mut res = vec![];
    for line in raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let splits = line
            .split(",")
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|e| eprintln!("ERROR: Failed to parse line: {e}"))?;
        if splits.len() != 2 {
            eprintln!("ERROR: Line should be X, Y.");
            return Err(());
        }
        res.push(splits[1] * GRID_SIZE + splits[0]);
    }
    Ok(res)
}

fn build_map(barriers: &[usize]) -> CharArray {
    let mut contents = vec![DOT_CHAR; GRID_SIZE * GRID_SIZE];
    for &c in barriers {
        contents[c] = BARRIER_CHAR;
    }
    CharArray {
        width: GRID_SIZE,
        height: GRID_SIZE,
        contents,
    }
}

impl CharArray {
    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    fn at(&self, y: i32, x: i32) -> Option<(usize, u8)> {
        if !self.is_valid(x, y) {
            None
        } else {
            let cursor = y as usize * self.width + x as usize;
            Some((cursor, self[cursor]))
        }
    }

    fn coordinates(&self, cursor: usize) -> (usize, usize) {
        (cursor / self.width, cursor % self.width)
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

fn dijkstra(map: &CharArray, start: usize, end: usize) -> HashMap<usize, (usize, Option<usize>)> {
    let mut q = BTreeSet::new();
    let mut dist_prev = HashMap::new();
    dist_prev.insert(start, (0, None));
    q.insert((0_usize, start));

    while let Some((d, u)) = q.pop_first() {
        if u == end {
            break;
        }
        let (uy, ux) = map.coordinates(u);
        for (dy, dx) in DIRECTIONS {
            if let Some((nc, c)) = map.at(uy as i32 + dy, ux as i32 + dx) {
                if c == BARRIER_CHAR {
                    continue;
                }
                let &(dv, _) = dist_prev.get(&nc).unwrap_or(&(usize::MAX, None));
                let new_dist = d.checked_add(1).unwrap_or(usize::MAX);
                if new_dist < dv {
                    dist_prev.insert(nc, (new_dist, Some(u)));
                    if dv != usize::MAX {
                        // v was visited, hence in Q
                        q.remove(&(dv, nc));
                    }
                    q.insert((new_dist, nc));
                }
            }
        }
    }
    dist_prev
}

fn build_optimal_path(
    end_state: usize,
    prev: &HashMap<usize, (usize, Option<usize>)>,
) -> HashSet<usize> {
    let mut path = HashSet::new();
    path.insert(end_state);
    let mut c = end_state;
    while let Some((_, Some(p))) = prev.get(&c) {
        path.insert(*p);
        c = *p;
    }
    path
}

fn display_optimal_path(map: &CharArray, path: &HashSet<usize>, next_bar: Option<usize>) {
    let mut display_map = map.clone();
    for &c in path {
        display_map[c] = b'O';
    }
    if let Some(c) = next_bar {
        display_map[c] = b'v';
    }
    println!("{display_map}");
}

fn first_block(
    map: &mut CharArray,
    barriers: &[usize],
    path: &HashSet<usize>,
) -> Option<(usize, usize)> {
    let mut path = path.to_owned();
    for &barrier in barriers {
        map[barrier] = BARRIER_CHAR;
        if !path.contains(&barrier) {
            continue;
        }
        let dist_prev = dijkstra(map, START_POS, END_POS);
        if !dist_prev.contains_key(&END_POS) {
            display_optimal_path(map, &path, Some(barrier));
            return Some((barrier % map.width, barrier / map.width));
        }
        path = build_optimal_path(END_POS, &dist_prev);
    }
    None
}

fn main() {
    let barriers = parse_barriers().unwrap();
    let mut map = build_map(&barriers[..PART_ONE_BARRIERS]);

    let dist_prev = dijkstra(&map, START_POS, END_POS);
    let (fst, _) = dist_prev.get(&END_POS).unwrap();
    let path = build_optimal_path(END_POS, &dist_prev);
    display_optimal_path(&map, &path, None);
    println!("Day 16, part 1: {fst}");

    let (snd_x, snd_y) = first_block(&mut map, &barriers[PART_ONE_BARRIERS..], &path)
        .expect("ERROR: Failed to find any barrier config that blocked all exit paths.");
    println!("Day 16, part 2: {snd_x},{snd_y}");
}
