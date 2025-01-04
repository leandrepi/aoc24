use std::{fmt, fs};

const BOX_CHAR: u8 = "O".as_bytes()[0];
const BARRIER_CHAR: u8 = "#".as_bytes()[0];
const PLAYER_CHAR: u8 = "@".as_bytes()[0];
const DOT_CHAR: u8 = ".".as_bytes()[0];
const LBOX_CHAR: u8 = "[".as_bytes()[0];
const RBOX_CHAR: u8 = "]".as_bytes()[0];
const SCORE_FACTOR: usize = 100;

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

fn parse_input() -> Result<(CharArray, String), ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    let raw = raw.trim();
    let mut parts = raw.split("\n\n");
    let map = CharArray::from(parts.next().expect("first section"))?;
    let mut moves = parts.next().expect("moves").to_owned();
    moves.retain(|c| !c.is_whitespace());
    Ok((map, moves))
}

fn find_player(map: &CharArray) -> Option<usize> {
    for (c, &x) in map.contents.iter().enumerate() {
        if x == PLAYER_CHAR {
            return Some(c);
        }
    }
    None
}

fn apply_move(map: &mut CharArray, r#move: char, player: &mut usize) {
    match r#move {
        '>' => horizontal_move(map, player, 1),
        '<' => horizontal_move(map, player, -1),
        '^' => vertical_move(map, player, -1),
        'v' => vertical_move(map, player, 1),
        _ => unreachable!(),
    };
}

fn horizontal_move(map: &mut CharArray, player: &mut usize, dir_x: i32) {
    let mut c = *player as i32 + dir_x;
    let c_min = (*player - *player % map.width) as i32;
    let c_max = c_min + map.width as i32;

    // move along the row until a barrier or an empty slot is found
    while c >= c_min && c < c_max {
        match map[c as usize] {
            BARRIER_CHAR => {
                return;
            }
            DOT_CHAR => {
                break;
            }
            _ => {
                c += dir_x;
            }
        }
    }

    // push the whole row towards the empty slot
    while c != *player as i32 {
        let next = c - dir_x;
        map.contents.swap(c as usize, next as usize);
        c = next;
    }

    // update player position
    *player = (c + dir_x) as usize;
}

fn to_move_vertically(
    map: &mut CharArray,
    player: &mut usize,
    dir_y: i32,
    indices: &mut Vec<usize>,
) {
    let y = *player / map.width;
    let x = *player % map.width;
    let ny = y as i32 + dir_y;

    let nplayer = x + ny as usize * map.width;

    // avoid unnecessary rechecks
    if indices.contains(&nplayer) {
        return;
    }

    // should never happen because the map is supposed to have a border of barriers
    if !map.is_valid(x as i32, ny) {
        indices.clear();
        return;
    }

    let to_push = match map[nplayer] {
        BARRIER_CHAR => {
            indices.clear();
            return;
        }
        DOT_CHAR => return,
        BOX_CHAR => nplayer,
        x if x == LBOX_CHAR || x == RBOX_CHAR => {
            indices.push(nplayer);
            *player = nplayer;
            to_move_vertically(map, player, dir_y, indices);
            if indices.is_empty() {
                return; // hit a barrier, short circuit
            }
            if x == LBOX_CHAR {
                nplayer + 1
            } else {
                nplayer - 1
            }
        }
        _ => unreachable!(),
    };
    indices.push(to_push);
    *player = to_push;
    to_move_vertically(map, player, dir_y, indices);
}

fn vertical_move(map: &mut CharArray, player: &mut usize, dir_y: i32) {
    let original_player = *player;

    // find movable squares, starting from the player position
    let mut indices = vec![original_player];
    to_move_vertically(map, player, dir_y, &mut indices);

    // restore the player position that was overwritten by to_move_vertically
    *player = original_player;

    // cannot push anything
    if indices.is_empty() {
        return;
    }

    // sort locations lexicographically, with the y in the opposite direction of dir_y
    let mut locs: Vec<(i32, i32)> = indices
        .iter()
        .map(|c| ((c % map.width) as i32, (c / map.width) as i32))
        .collect();
    locs.sort_by(|(ax, ay), (bx, by)| (ax, -dir_y * ay).cmp(&(bx, -dir_y * by)));

    // pull the entire rows from the eventual empty slot
    for (x, y) in locs {
        let cur = x as usize + y as usize * map.width;
        map.contents
            .swap(cur, x as usize + ((y + dir_y) as usize) * map.width);
    }

    // update player position
    *player = (*player as i32 + dir_y * map.width as i32) as usize;
}

fn apply_moves(map: &mut CharArray, moves: &str) {
    let mut player = find_player(map).expect("need at least a player");
    for r#move in moves.chars() {
        apply_move(map, r#move, &mut player);
    }
}

fn box_count(map: &CharArray) -> usize {
    map.contents
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == BOX_CHAR || c == LBOX_CHAR)
        .map(|(c, _)| c / map.width * SCORE_FACTOR + c % map.width)
        .sum()
}

fn map_part_two(map: &CharArray) -> CharArray {
    let mut contents = vec![0; map.contents.len() * 2];
    for (c, &x) in map.contents.iter().enumerate() {
        let (fst, snd) = match x {
            BOX_CHAR => (LBOX_CHAR, RBOX_CHAR),
            PLAYER_CHAR => (PLAYER_CHAR, DOT_CHAR),
            y => (y, y),
        };
        contents[c * 2] = fst;
        contents[c * 2 + 1] = snd;
    }
    CharArray {
        contents,
        width: map.width * 2,
        height: map.height * 2,
    }
}

fn main() {
    let (map, moves) = parse_input().unwrap();
    let mut map_one = map.clone();
    apply_moves(&mut map_one, &moves);
    let fst = box_count(&map_one);
    println!("Day 15, part 1: {}", fst);

    let mut map_two = map_part_two(&map);
    apply_moves(&mut map_two, &moves);
    let snd = box_count(&map_two);
    println!("Day 15, part 2: {}", snd);
}
