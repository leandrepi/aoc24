use std::{collections::HashMap, fs};

const NUMERIC_HEIGHT: usize = 4;
const NUMERIC_WIDTH: usize = 3;
const INVALID: char = 'x';
const NUMERIC_KEYPAD: [char; NUMERIC_HEIGHT * NUMERIC_WIDTH] = [
    '7', '8', '9', '4', '5', '6', '1', '2', '3', INVALID, '0', 'A',
];
const DIRECTIONAL_HEIGHT: usize = 2;
const DIRECTIONAL_WIDTH: usize = 3;
const DIRECTIONAL_KEYPAD: [char; DIRECTIONAL_HEIGHT * DIRECTIONAL_WIDTH] =
    [INVALID, '^', 'A', '<', 'v', '>'];
const START: char = 'A';
const BUTTON_PUSH: char = 'A';

pub struct Code {
    keys: String,
}

// for a given directional or numerical array, for each start element x,
// give all best move sequences from x to y in array
// best moves mean at most one direction change
fn next_move_lut(array: &[char], width: usize) -> HashMap<char, HashMap<char, Vec<String>>> {
    let mut res = HashMap::new();
    let to_avoid = array.iter().position(|&c| c == INVALID).unwrap();
    let to_avoid_x = (to_avoid % width) as i32;
    let to_avoid_y = (to_avoid / width) as i32;
    for (idx, &key) in array.iter().enumerate() {
        if key == INVALID {
            continue;
        }
        let x = idx % width;
        let y = idx / width;
        let mut key_map = HashMap::new();
        for (jdx, &other) in array.iter().enumerate() {
            if other == INVALID {
                continue;
            }
            let jx = jdx % width;
            let jy = jdx / width;
            let dx = jx as i32 - x as i32;
            let dy = jy as i32 - y as i32;
            let to_push_x = if dx > 0 { '>' } else { '<' };
            let to_push_y = if dy > 0 { 'v' } else { '^' };

            let mut seq = String::new();
            for _ in 0..(dy.abs()) {
                seq.push(to_push_y);
            }
            for _ in 0..(dx.abs()) {
                seq.push(to_push_x);
            }
            if dx == 0 || dy == 0 {
                seq.push(BUTTON_PUSH);
                key_map.insert(other, vec![seq]);
                continue;
            }
            key_map.insert(other, vec![]);
            let mut rev_seq = seq.chars().rev().collect::<String>();
            if (y as i32 + dy, x as i32) != (to_avoid_y, to_avoid_x) {
                seq.push(BUTTON_PUSH);
                key_map.entry(other).and_modify(|v| v.push(seq));
            }
            if (y as i32, x as i32 + dx) != (to_avoid_y, to_avoid_x) {
                rev_seq.push(BUTTON_PUSH);
                key_map.entry(other).and_modify(|v| v.push(rev_seq));
            }
        }
        res.insert(key, key_map);
    }
    res
}

fn next_dir(keys: &str, lut: &HashMap<char, HashMap<char, Vec<String>>>) -> Vec<String> {
    let mut res = vec![String::new()];
    let mut cur = START;
    for c in keys.chars() {
        let moves = lut.get(&cur).unwrap().get(&c).unwrap();
        let end = res.len();
        assert!(moves.len() <= 2, "At most two moves from x to y");
        if moves.len() == 2 {
            let mut res_clone = res.clone();
            for acc in &mut res_clone {
                acc.push_str(&moves[1]);
            }
            res.extend_from_slice(&res_clone);
        }
        for acc in &mut res[..end].iter_mut() {
            acc.push_str(&moves[0]);
        }
        cur = c;
    }
    res
}

fn shortest_sequence(
    directional: &str,
    lut: &HashMap<char, HashMap<char, Vec<String>>>,
    cache: &mut HashMap<(String, usize), usize>,
    max_depth: usize,
    depth: usize,
) -> usize {
    let cache_dir = directional.to_owned();
    if let Some(&res) = cache.get(&(cache_dir.clone(), depth)) {
        return res;
    }
    if depth == max_depth {
        let res = directional.len();
        cache.insert((cache_dir, depth), res);
        return res;
    }
    let mut shortest = usize::MAX;
    for next in &next_dir(directional, lut) {
        let mut res = 0;
        for cmd in next[..(next.len() - 1)]
            .split(BUTTON_PUSH)
            .map(|s| format!("{s}{BUTTON_PUSH}"))
        {
            let sub = shortest_sequence(&cmd, lut, cache, max_depth, depth + 1);
            cache.insert((cmd, depth + 1), sub);
            res += sub;
        }
        if res < shortest {
            shortest = res
        }
    }
    cache.insert((cache_dir, depth), shortest);
    shortest
}

impl Code {
    pub fn num_part(&self) -> usize {
        self.keys
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .expect("Valid digits")
    }
}

fn parse_input() -> Result<Vec<Code>, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    Ok(raw
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Code { keys: l.to_owned() })
        .collect())
}

fn complexities(codes: &[Code], max_depth: usize) -> usize {
    let num_lut = next_move_lut(&NUMERIC_KEYPAD, NUMERIC_WIDTH);
    let dir_lut = next_move_lut(&DIRECTIONAL_KEYPAD, DIRECTIONAL_WIDTH);
    let mut cache = HashMap::new();
    let mut total = 0;
    for code in codes {
        let mut shortest = usize::MAX;
        for dir in next_dir(&code.keys, &num_lut) {
            let res = shortest_sequence(&dir, &dir_lut, &mut cache, max_depth, 0);
            if res < shortest {
                shortest = res
            }
        }
        total += code.num_part() * shortest;
    }
    total
}

fn main() {
    let codes = parse_input().unwrap();
    let fst = complexities(&codes, 2);
    println!("Day 21, part 1: {fst}");
    let snd = complexities(&codes, 25);
    println!("Day 21, part 2: {snd}");
}
