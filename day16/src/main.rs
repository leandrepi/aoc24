use std::{collections::HashSet, fmt, fs};

const BARRIER_CHAR: u8 = "#".as_bytes()[0];
const START_CHAR: u8 = "S".as_bytes()[0];
const END_CHAR: u8 = "E".as_bytes()[0];
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

impl CharArray {
    fn from(raw: &str) -> Result<Self, ()> {
        let mut lines = raw.lines().map(|l| l.trim()).filter(|l| l.len() > 0);
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
        if start != None && end != None {
            return (start.unwrap(), end.unwrap());
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

fn rotate_direction(direction_index: usize) -> Vec<usize> {
    assert!(
        direction_index < DIRECTIONS.len(),
        "Can only use directions within the const array EWNS"
    );
    let (_, dir_x) = DIRECTIONS[direction_index];
    let rot = if dir_x == 0 {
        [(0, 1), (0, -1)]
    } else {
        [(1, 0), (-1, 0)]
    };
    rot.iter()
        .map(|&c| DIRECTIONS.iter().position(|&cc| cc == c).unwrap())
        .collect()
}

fn update_dist(dist: &mut [usize], prev: &mut [Vec<usize>], cost: usize, cur: usize, next: usize) {
    let alt = dist[cur] + cost;
    if alt < dist[next] {
        dist[next] = alt;
        prev[next] = vec![cur];
    } else if alt == dist[next] {
        prev[next].push(cur);
    }
}

fn dijkstra(map: &CharArray, start: usize, end: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut dist = vec![];
    let mut prev = vec![];
    let mut q = HashSet::new();
    for _ in 0..(map.contents.len() * DIRECTIONS.len()) {
        dist.push(usize::MAX);
        prev.push(vec![]);
    }
    dist[start * DIRECTIONS.len()
        + DIRECTIONS
            .iter()
            .position(|&(dir_y, dir_x)| dir_y == 0 && dir_x == 1)
            .unwrap()] = 0;

    loop {
        let (u, _) = dist
            .iter()
            .enumerate()
            .filter(|(i, _)| !q.contains(i))
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .expect("q is not empty");

        let direction = u % DIRECTIONS.len();
        let cursor = u / DIRECTIONS.len();
        if cursor == end {
            break;
        }

        q.insert(u);

        let (dir_y, dir_x) = DIRECTIONS[direction];
        let ux = cursor % map.width;
        let uy = cursor / map.width;

        let nx = ux as i32 + dir_x;
        let ny = uy as i32 + dir_y;
        if map.is_valid(nx, ny) {
            let c = ny as usize * map.width + nx as usize;
            let cd = c * DIRECTIONS.len() + direction;
            if map[c] != BARRIER_CHAR && !q.contains(&cd) {
                update_dist(&mut dist, &mut prev, 1, u, cd);
            }
        }

        for rot_dir in rotate_direction(direction) {
            let c = cursor * DIRECTIONS.len() + rot_dir;
            if q.contains(&c) {
                continue;
            }
            update_dist(&mut dist, &mut prev, 1000, u, c);
        }
    }
    (dist, prev)
}

fn visit_optimal(prev: &[Vec<usize>], cur: usize, marked: &mut HashSet<usize>) {
    marked.insert(cur / DIRECTIONS.len());
    for &p in prev[cur].iter() {
        visit_optimal(prev, p, marked);
    }
}

fn min_cost_end_state(dist: &[usize], end: usize) -> (usize, usize) {
    let end_start = end * DIRECTIONS.len();
    let mut fst = usize::MAX;
    let mut end_idx = end_start;
    for (i, &c) in dist[end_start..end_start + DIRECTIONS.len()]
        .iter()
        .enumerate()
    {
        if c < fst {
            fst = c;
            end_idx = end_start + i;
        }
    }
    (fst, end_idx)
}

fn display_optimal_path(map: &CharArray, prev: &[Vec<usize>], end_idx: usize) {
    let mut display_map = map.clone();
    let mut c = end_idx;
    loop {
        if prev[c].len() == 0 {
            break;
        }
        let p = prev[c][0];
        display_map[c / DIRECTIONS.len()] = match DIRECTIONS[c % DIRECTIONS.len()] {
            (0, 1) => ">",
            (0, -1) => "<",
            (1, 0) => "v",
            (-1, 0) => "^",
            _ => unreachable!(),
        }
        .as_bytes()[0];
        c = p;
    }
    display_map[end_idx / DIRECTIONS.len()] = "E".as_bytes()[0];
    display_map[c / DIRECTIONS.len()] = "S".as_bytes()[0];
    println!("{display_map}");
}

fn main() {
    // The input example is so slooooooooooooooooooooooooooow
    let raw = fs::read_to_string("example.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let mut map = CharArray::from(&raw).unwrap();

    let (start, end) = find_start(&map);
    let (dist, prev) = dijkstra(&map, start, end);

    let (fst, end_state) = min_cost_end_state(&dist, end);
    let mut marked = HashSet::new();
    visit_optimal(&prev, end_state, &mut marked);
    let snd = marked.len();

    display_optimal_path(&map, &prev, end_state);
    println!("Day 16, part 1: {fst}");
    for &c in &marked {
        map[c] = "o".as_bytes()[0];
    }
    println!("{map}");
    println!("Day 16, part 2: {snd}");
}
