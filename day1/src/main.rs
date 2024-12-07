use std::{collections::HashMap, fs};

fn parse_input(file_path: &str) -> Result<(Vec<u32>, Vec<u32>), ()> {
    let data = fs::read_to_string(file_path).map_err(|e| eprintln!("Failed to read file: {e}"))?;
    let mut xs: Vec<u32> = vec![];
    let mut ys: Vec<u32> = vec![];
    for (line_idx, line) in data.lines().enumerate() {
        let splits: Vec<u32> = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|&l| l.len() > 0)
            .map(|s| s.parse())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| {
                eprintln!(
                    "{file_path}:{row}: Failed to parse line as a list of integers: {e}.",
                    row = line_idx + 1
                );
            })?;
        if splits.len() != 2 {
            eprintln!(
                "{file_path}:{row}: Invalid line, found {_len} values instead of 2.",
                row = line_idx + 1,
                _len = splits.len()
            );
            return Err(());
        }
        xs.push(splits[0]);
        ys.push(splits[1]);
    }
    Ok((xs, ys))
}

fn part1(xs: &mut [u32], ys: &mut [u32]) -> u32 {
    xs.sort();
    ys.sort();
    xs.iter()
        .zip(ys.iter())
        .map(|(&a, &b)| (a as i32 - b as i32).abs() as u32)
        .sum()
}

fn part2(xs: &[u32], ys: &[u32]) -> u32 {
    let mut y_counts = HashMap::new();
    for c in ys {
        y_counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    xs.iter().map(|c| c * *y_counts.entry(c).or_insert(0)).sum()
}

fn main() {
    let (mut xs, mut ys) = parse_input("input.txt").unwrap();
    let first = part1(&mut xs, &mut ys);
    println!("Day 1, part1 : {first}");
    let second = part2(&xs, &ys);
    println!("Day 1, part2 : {second}");
}
