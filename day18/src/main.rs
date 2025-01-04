use std::{collections::HashSet, fmt, fs};

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

fn update_dist(
    dist: &mut [usize],
    prev: &mut [Option<usize>],
    cost: usize,
    cur: usize,
    next: usize,
) {
    let alt = dist[cur].checked_add(cost).unwrap_or(usize::MAX);
    if alt < dist[next] {
        dist[next] = alt;
        prev[next] = Some(cur);
    }
}

fn dijkstra(map: &CharArray, start: usize, end: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let mut dist = Vec::with_capacity(map.contents.len());
    let mut prev = Vec::with_capacity(map.contents.len());
    let mut q = HashSet::new();
    for v in 0..(map.contents.len()) {
        dist.push(usize::MAX);
        prev.push(None);
        q.insert(v);
    }
    dist[start] = 0;

    while !q.is_empty() {
        let (u, _) = q
            .iter()
            .map(|&x| (x, dist[x]))
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .expect("q is not empty");
        q.remove(&u);

        if u == end {
            break;
        }

        let ux = u % map.width;
        let uy = u / map.width;

        for (dir_y, dir_x) in DIRECTIONS {
            let nx = ux as i32 + dir_x;
            let ny = uy as i32 + dir_y;

            if map.is_valid(nx, ny) {
                let c = ny as usize * map.width + nx as usize;
                if map[c] != BARRIER_CHAR && q.contains(&c) {
                    update_dist(&mut dist, &mut prev, 1, u, c);
                }
            }
        }
    }
    (dist, prev)
}

fn build_optimal_path(prev: &[Option<usize>], end: usize) -> Vec<usize> {
    let mut path = vec![];
    let mut c = end;
    path.push(end);
    while let Some(p) = prev[c] {
        c = p;
        path.push(c);
    }
    path
}

fn display_optimal_path(map: &CharArray, path: &[usize], next_bar: Option<usize>) {
    let mut display_map = map.clone();
    for &c in path {
        display_map[c] = "O".as_bytes()[0];
    }
    if let Some(c) = next_bar {
        display_map[c] = "X".as_bytes()[0];
    }
    println!("{display_map}");
}

fn first_block(map: &mut CharArray, barriers: &[usize], path: &[usize]) -> Option<(usize, usize)> {
    let mut path = path.to_owned();
    for &barrier in barriers {
        map[barrier] = BARRIER_CHAR;
        if !path.contains(&barrier) {
            continue;
        }
        let (dist, prev) = dijkstra(map, START_POS, END_POS);
        if dist[END_POS] == usize::MAX {
            display_optimal_path(map, &path, Some(barrier));
            return Some((barrier % map.width, barrier / map.width));
        }
        path = build_optimal_path(&prev, END_POS);
    }
    None
}

fn main() {
    let barriers = parse_barriers().unwrap();
    let mut map = build_map(&barriers[..PART_ONE_BARRIERS]);

    let (dist, prev) = dijkstra(&map, START_POS, END_POS);
    let fst = dist[END_POS];
    let path = build_optimal_path(&prev, END_POS);
    display_optimal_path(&map, &path, None);
    println!("Day 16, part 1: {fst}");

    let (snd_x, snd_y) = first_block(&mut map, &barriers[PART_ONE_BARRIERS..], &path)
        .expect("ERROR: Failed to find any barrier config that blocked all exit paths.");
    println!("Day 16, part 2: {snd_x},{snd_y}");
}
