use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct CharArray {
    contents: Vec<u8>,
    width: usize,
    height: usize,
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

pub struct Day6Player {
    y: i32,
    x: i32,
    dir_y: i32,
    dir_x: i32,
    cursor: u8,
}

fn find_player(contents: &CharArray, chars: &str) -> Result<Day6Player, ()> {
    let mut char_map = HashMap::new();
    let to_match = chars.bytes().collect::<Vec<u8>>();
    for (c, uc) in chars.chars().zip(to_match.iter()) {
        char_map.insert(uc, c);
    }
    for y in 0..contents.height {
        for x in 0..contents.width {
            // if chars.itercontains(contents[y * contents.width + x]):
            let current = &contents[y * contents.width + x];
            if let Some(c) = to_match.iter().find(|&c| c == current) {
                let (dir_y, dir_x) = match char_map.get(c) {
                    Some('>') => (0, 1),
                    Some('v') => (1, 0),
                    Some('<') => (0, -1),
                    Some('^') => (-1, 0),
                    _ => unreachable!(),
                };
                return Ok(Day6Player {
                    y: y as i32,
                    x: x as i32,
                    dir_y,
                    dir_x,
                    cursor: *c,
                });
            }
        }
    }
    eprintln!("ERROR: Could not find player in map");
    Err(())
}

fn walk_map(contents: &mut CharArray) -> Result<Option<u32>, ()> {
    let mut player = find_player(contents, "><^v")?;
    let mut result = 0;
    let barrier = "#".bytes().next().expect("ascii");
    let cursors = "><^v".bytes().collect::<Vec<u8>>();
    'outer: loop {
        let cur = (player.y as usize) * contents.width + player.x as usize;
        if contents[cur] == player.cursor {
            if result != 0 {
                return Ok(None);
            }
            result += 1;
        } else if !cursors.contains(&contents[cur]) {
            contents[cur] = player.cursor;
            result += 1;
        }
        let mut dir_x = player.dir_x;
        let mut dir_y = player.dir_y;
        let mut nx = player.x + dir_x;
        let mut ny = player.y + dir_y;
        if nx < 0 || (nx as usize) >= contents.width || ny < 0 || (ny as usize) >= contents.height {
            break;
        }
        let mut tries = 4;
        while contents[(ny as usize) * contents.width + (nx as usize)] == barrier && tries > 0 {
            (dir_y, dir_x) = match (dir_y, dir_x) {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
            nx = player.x as i32 + dir_x;
            ny = player.y as i32 + dir_y;
            if nx < 0
                || (nx as usize) >= contents.width
                || ny < 0
                || (ny as usize) >= contents.height
            {
                break 'outer;
            }
            tries -= 1;
        }
        if tries == 0 {
            eprintln!("ERROR: player is stuck.");
            return Err(());
        }
        let cursor = match (dir_y, dir_x) {
            (0, 1) => ">",
            (1, 0) => "v",
            (0, -1) => "<",
            (-1, 0) => "^",
            _ => unreachable!(),
        }
        .bytes()
        .next()
        .expect("ascii");
        player.x = nx;
        player.y = ny;
        player.dir_y = dir_y;
        player.dir_x = dir_x;
        player.cursor = cursor;
    }
    Ok(Some(result))
}

fn reset_map(map: &mut CharArray, start_pos: &Day6Player) {
    let barrier = "#".bytes().next().expect("ascii");
    let dot = ".".bytes().next().expect("ascii");
    for cursor in 0..map.contents.len() {
        if map[cursor] != barrier {
            map[cursor] = dot;
        }
    }
    let cursor = (start_pos.y as usize) * map.width + (start_pos.x as usize);
    map[cursor] = start_pos.cursor;
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let mut map = CharArray::from(&raw);
    let pos_chars = "><^v";
    let pos_bytes = pos_chars.bytes().collect::<Vec<u8>>();
    let start_pos = find_player(&map, pos_chars).unwrap();

    let result_part1 = walk_map(&mut map).unwrap().unwrap_or_default();
    println!("Day 6, part 1: {result_part1}");

    let start_cur = start_pos.y as usize * map.width + start_pos.x as usize;
    let candidates = map
        .contents
        .iter()
        .enumerate()
        .filter(|(i, c)| *i != start_cur && pos_bytes.contains(c))
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let mut result_part2 = 0;
    let barrier = "#".bytes().next().expect("ascii");
    for cur in candidates {
        reset_map(&mut map, &start_pos);
        let prev = map[cur];
        map[cur] = barrier;
        if let Ok(None) = walk_map(&mut map) {
            result_part2 += 1;
        }
        map[cur] = prev;
    }
    println!("Day 6, part 2: {result_part2}");
}
