use std::{collections::VecDeque, fs};
const PRUNE_MOD: u64 = 16777216;
const FST_MULT_LOG2: u64 = 6; // * 64 equiv << 6
const DIV_LOG2: u64 = 5; // / 32 equiv >> 5
const SND_MULT_LOG2: u64 = 11; // * 2048 equiv << 11
const MAX_ITER: usize = 2000;
const MONKEY_CHANGES: usize = 4;
const MIN_CHANGE: i8 = -9;
const MAX_CHANGE: i8 = 9;
const CHANGE_BASE: i8 = MAX_CHANGE - MIN_CHANGE + 1;
const TOTAL_VALUES: usize = (CHANGE_BASE as usize).pow(MONKEY_CHANGES as u32);

fn mix_and_prune(secret: u64, tmp: u64) -> u64 {
    (secret ^ tmp) % PRUNE_MOD
}

fn update_secret(secret: u64) -> u64 {
    let secret = mix_and_prune(secret, secret << FST_MULT_LOG2);
    let secret = mix_and_prune(secret, secret >> DIV_LOG2);
    mix_and_prune(secret, secret << SND_MULT_LOG2)
}

fn key_to_idx(key: &[i8; 4]) -> usize {
    let mut res = 0;
    for item in key {
        res *= CHANGE_BASE as usize;
        res += (item - MIN_CHANGE) as usize;
    }
    res
}

fn iter_update(secret: u64, n: usize, monkeys: &mut [u16; TOTAL_VALUES]) -> u64 {
    let mut secret = secret;
    let mut changes = VecDeque::with_capacity(MONKEY_CHANGES);
    let mut seen = [false; TOTAL_VALUES];
    let mut prev = (secret % 10) as u16;
    for _ in 0..n {
        secret = update_secret(secret);
        let prize = (secret % 10) as u16;
        changes.push_back(prize as i8 - prev as i8);
        if changes.len() == MONKEY_CHANGES {
            let key = [changes[0], changes[1], changes[2], changes[3]];
            let idx = key_to_idx(&key);
            changes.pop_front();
            if !seen[idx] {
                monkeys[idx] += prize;
            }
            seen[idx] = true;
        }
        prev = prize;
    }
    secret
}

fn parse_input() -> Result<Vec<u64>, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    raw.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| eprintln!("Failed to parse row as int: {e}"))
}

fn main() {
    let secrets = parse_input().unwrap();
    let mut monkeys = [0; TOTAL_VALUES];
    let fst: u64 = secrets
        .iter()
        .map(|&s| iter_update(s, MAX_ITER, &mut monkeys))
        .sum();
    println!("Day 22, part 1: {}", fst);
    let snd = monkeys.iter().max().unwrap();
    println!("Day 22, part 2: {}", snd);
}
