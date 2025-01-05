use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

const BARRIER_CHAR: u8 = b'#';
const START_CHAR: u8 = b'S';
const END_CHAR: u8 = b'E';
const ROT_COST: usize = 1000;
const FWD_COST: usize = 1;

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

impl std::fmt::Display for CharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct DijkstraState {
    node: usize,
    dir: (i32, i32),
}

impl DijkstraState {
    fn new(node: usize, dir: (i32, i32)) -> Self {
        Self { node, dir }
    }

    fn next_neighbors(&self, map: &CharArray) -> Vec<(Self, usize)> {
        let mut neighbors = vec![];
        let (y, x) = map.coordinates(self.node);
        let y = y as i32;
        let x = x as i32;
        let (dy, dx) = self.dir;

        if let Some((nc, c)) = map.at(y + dy, x + dx) {
            if c != BARRIER_CHAR {
                neighbors.push((
                    Self {
                        node: nc,
                        dir: self.dir,
                    },
                    FWD_COST,
                ));
            }
        }
        let turns = match dy {
            0 => [(1, 0), (-1, 0)],
            _ => [(0, 1), (0, -1)],
        };
        for (dy, dx) in turns {
            neighbors.push((Self::new(self.node, (dy, dx)), ROT_COST));
        }
        neighbors
    }
}

fn dijkstra(
    map: &CharArray,
    start: usize,
    end: usize,
) -> (
    DijkstraState,
    HashMap<DijkstraState, (usize, Vec<DijkstraState>)>,
) {
    let mut q = BTreeSet::new();
    let mut dist_prev = HashMap::new();
    let node = DijkstraState::new(start, (0, 1));
    dist_prev.insert(node, (0, vec![]));
    q.insert((0, node));

    let mut end_state = None;
    while let Some((d, u)) = q.pop_first() {
        if u.node == end {
            end_state = Some(u);
            break;
        }
        for (neighbor, cost) in u.next_neighbors(map) {
            let &(dv, _) = dist_prev.get(&neighbor).unwrap_or(&(usize::MAX, vec![]));
            let new_dist = cost.checked_add(d).unwrap_or(usize::MAX);
            match new_dist.cmp(&dv) {
                Ordering::Less => {
                    dist_prev.insert(neighbor, (new_dist, vec![u]));
                    if dv != usize::MAX {
                        // v was visited, hence in Q
                        q.remove(&(dv, neighbor));
                    }
                    q.insert((new_dist, neighbor));
                }
                Ordering::Equal => {
                    dist_prev.entry(neighbor).and_modify(|(_, v)| v.push(u));
                }
                _ => (),
            }
        }
    }
    (end_state.unwrap(), dist_prev)
}

fn build_optimal_path(
    end_state: DijkstraState,
    prev: &HashMap<DijkstraState, (usize, Vec<DijkstraState>)>,
) -> Vec<(usize, (i32, i32))> {
    let mut path = vec![];
    let mut c = end_state;
    while let Some(p) = prev.get(&c).unwrap().1.first() {
        path.push((p.node, p.dir));
        c = *p;
    }
    path
}

fn display_optimal_path(map: &CharArray, path: &[(usize, (i32, i32))]) {
    let mut display_map = map.clone();
    for &(c, (dir_y, dir_x)) in path[..path.len() - 1].iter() {
        display_map[c] = match (dir_y, dir_x) {
            (1, 0) => b'v',
            (-1, 0) => b'^',
            (0, 1) => b'>',
            _ => b'<',
        };
    }
    println!("{display_map}");
}

fn visit_optimal(
    prev: &HashMap<DijkstraState, (usize, Vec<DijkstraState>)>,
    cur: DijkstraState,
    marked: &mut HashSet<usize>,
) {
    marked.insert(cur.node);
    for &p in prev.get(&cur).unwrap().1.iter() {
        visit_optimal(prev, p, marked);
    }
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let mut map = CharArray::from(&raw).unwrap();

    let (start, end) = find_start(&map);
    let (end_state, dist_prev) = dijkstra(&map, start, end);

    let &(fst, _) = dist_prev.get(&end_state).unwrap();
    let mut marked = HashSet::new();
    visit_optimal(&dist_prev, end_state, &mut marked);
    let snd = marked.len();

    let path = build_optimal_path(end_state, &dist_prev);
    display_optimal_path(&map, &path);
    println!("Day 16, part 1: {fst}");
    for &c in &marked {
        map[c] = b'o';
    }
    println!("{map}");
    println!("Day 16, part 2: {snd}");
}
