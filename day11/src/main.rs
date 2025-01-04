use std::{collections::HashMap, fs};

fn parse_input() -> Result<Vec<u64>, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    let line = raw
        .lines()
        .find(|&l| !l.trim().is_empty())
        .expect("ERROR: Need at least a line to parse.");
    let stones = line
        .split(" ")
        .map(|s| s.trim())
        .filter(|&c| !c.is_empty())
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| eprintln!("Failed to parse line as a list of integers: {e}.",))?;
    Ok(stones)
}

fn n_digits(n: u64) -> u32 {
    ((n as f64).log10().floor() as u32) + 1
}

fn stones_to_counts(stones: &[u64]) -> HashMap<u64, u64> {
    let mut blink_counts = HashMap::new();
    for &s in stones {
        blink_counts.entry(s).and_modify(|c| *c += 1).or_insert(1);
    }
    blink_counts
}

fn apply_rule(blink_counts: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_counts = HashMap::new();
    for (&s, &c) in blink_counts {
        let new_s = match (s, n_digits(s)) {
            (0, _) => 1,
            (s, n) if n % 2 == 0 => {
                let pow = 10u64.pow(n / 2);
                let tail = s % pow;
                new_counts.entry(tail).and_modify(|k| *k += c).or_insert(c);
                (s - tail) / pow
            }
            (s, _) => s * 2024,
        };
        new_counts.entry(new_s).and_modify(|k| *k += c).or_insert(c);
    }
    new_counts
}

fn iter_rule(blink_counts: HashMap<u64, u64>, n_blinks: u32) -> HashMap<u64, u64> {
    let mut counts = blink_counts;
    for _ in 0..n_blinks {
        counts = apply_rule(&counts);
    }
    counts
}

fn total_counts(counts: &HashMap<u64, u64>) -> u64 {
    counts.iter().map(|(_, c)| c).sum()
}

fn count_stones(stones: &[u64], blinks_part1: u32, blinks_part2: u32) -> (u64, u64) {
    let mut blink_counts = stones_to_counts(stones);
    blink_counts = iter_rule(blink_counts, blinks_part1);
    let fst = total_counts(&blink_counts);
    blink_counts = iter_rule(blink_counts, blinks_part2 - blinks_part1);
    let snd = total_counts(&blink_counts);
    (fst, snd)
}

fn main() {
    let stones = parse_input().unwrap();
    let (fst, snd) = count_stones(&stones, 25, 75);
    println!("Day 11, part 1: {fst}");
    println!("Day 11, part 2: {snd}");
}
