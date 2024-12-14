use std::fs;

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
        let mut lines = raw.lines().map(|l| l.trim()).filter(|l| l.len() > 0);
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

fn visit_region(
    map: &CharArray,
    start: usize,
    visited: &mut [bool],
    area: &mut u32,
    perimeter: &mut u32,
    boundaries: &mut Vec<(f32, f32)>,
) {
    if visited[start] {
        return;
    }
    *area += 1;
    let y = start / map.width;
    let x = start % map.width;
    visited[start] = true;
    for (dir_y, dir_x) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let nx = x as i32 + dir_x;
        let ny = y as i32 + dir_y;
        if map.is_valid(nx, ny) {
            let nc = ny as usize * map.width + nx as usize;
            if map[nc] == map[start] {
                if !visited[nc] {
                    visit_region(map, nc, visited, area, perimeter, boundaries);
                }
            } else {
                *perimeter += 1;
                boundaries.push((
                    x as f32 + (dir_x as f32) / 4.0,
                    y as f32 + (dir_y as f32) / 4.0,
                ))
            }
        } else {
            *perimeter += 1;
            boundaries.push((
                x as f32 + (dir_x as f32) / 4.0,
                y as f32 + (dir_y as f32) / 4.0,
            ))
        }
    }
}

fn count_sides(boundaries: &[(f32, f32)]) -> u32 {
    let mut vertical = 1;
    let mut boundaries: Vec<_> = boundaries
        .iter()
        .filter(|(bx, _)| bx.fract().abs() > 0.1)
        .collect();
    boundaries.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let (mut start_x, mut cur_y) = boundaries[0];
    for &(bx, by) in boundaries[1..].iter() {
        if *bx != start_x || *by > cur_y + 1.0 {
            vertical += 1;
        }
        cur_y = *by;
        start_x = *bx;
    }
    // for every vertical side there needs to be a horizontal one
    2 * vertical
}

fn map_price(map: &CharArray) -> (u32, u32) {
    let mut price_peri = 0;
    let mut price_sides = 0;
    let mut visited = vec![false; map.contents.len()];
    for cur in 0..map.contents.len() {
        if visited[cur] {
            continue;
        }
        let mut area = 0;
        let mut peri = 0;
        let mut boundaries = vec![];
        visit_region(
            map,
            cur,
            &mut visited,
            &mut area,
            &mut peri,
            &mut boundaries,
        );
        price_peri += area * peri;
        price_sides += area * count_sides(&boundaries);
    }
    (price_peri, price_sides)
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let map = CharArray::from(&raw);
    let (fst, snd) = map_price(&map);
    println!("Day 12, part 1: {fst}");
    println!("Day 12, part 2: {snd}");
}
