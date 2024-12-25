use std::{fmt, fs};

const PIN_CHAR: u8 = '#' as u8;
const DOT_CHAR: u8 = '.' as u8;
const MAX_HEIGHT: usize = 5;

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

    fn is_lock(&self) -> bool {
        self.contents[..self.width].iter().all(|&c| c == PIN_CHAR)
    }

    fn pin_heights(&self) -> Vec<usize> {
        let mut res = vec![];
        let c = if self.is_lock() { PIN_CHAR } else { DOT_CHAR };
        for x in 0..self.width {
            let mut h = 0;
            let mut cur = self.width + x;
            for _ in 0..MAX_HEIGHT {
                if self.contents[cur] != c {
                    break;
                }
                h += 1;
                cur += self.width;
            }
            res.push(if c == PIN_CHAR { h } else { MAX_HEIGHT - h });
        }
        res
    }
}

struct Patterns {
    locks: Vec<CharArray>,
    keys: Vec<CharArray>,
}

impl Patterns {
    fn fitting_pairs(&self) -> usize {
        let mut res = 0;
        for key in self.keys.iter() {
            let heights = key.pin_heights();
            res += self
                .locks
                .iter()
                .filter(|k| compatible_heights(&heights, &k.pin_heights()))
                .count()
        }
        res
    }
}

fn compatible_heights(lock_heights: &[usize], key_heights: &[usize]) -> bool {
    lock_heights
        .iter()
        .zip(key_heights.iter())
        .all(|(&l, &k)| l + k <= MAX_HEIGHT)
}

fn parse_input() -> Result<Patterns, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;

    let mut locks = vec![];
    let mut keys = vec![];
    for pattern in raw.trim().split("\n\n") {
        let pattern = CharArray::from(pattern.trim())?;
        if pattern.is_lock() {
            locks.push(pattern);
        } else {
            keys.push(pattern)
        }
    }
    Ok(Patterns { locks, keys })
}

fn main() {
    let patterns = parse_input().unwrap();
    let fst = patterns.fitting_pairs();
    println!("Day 25, part 1: {fst}");
}
