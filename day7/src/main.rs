use std::fs;

#[derive(Debug)]
pub struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

fn cat(l: u64, r: u64) -> u64 {
    let n_digits = ((r as f64).log10().floor() as u32) + 1;
    l * 10u64.pow(n_digits) + r
}

impl Equation {
    fn is_valid_part1(&self) -> bool {
        let n = self.rhs.len() - 1;
        for i in 0..(1 << n) {
            let mut res = self.rhs[0];
            for (k, r) in self.rhs[1..].iter().enumerate() {
                if (i >> k) & 1 == 1 {
                    res += r;
                } else {
                    res *= r;
                }
                if res > self.lhs {
                    break;
                }
            }
            if res == self.lhs {
                return true;
            }
        }
        false
    }
    fn is_valid_part2(&self) -> bool {
        let n = self.rhs.len() - 1;
        let m = 3u64.pow(n as u32);
        for i in 0..m {
            let mut res = self.rhs[0];
            for (k, r) in self.rhs[1..].iter().enumerate() {
                match (i / 3u64.pow(k as u32)) % 3 {
                    0 => {
                        res += r;
                    }
                    1 => {
                        res *= r;
                    }
                    2 => {
                        res = cat(res, *r);
                    }
                    _ => unreachable!(),
                }
                if res > self.lhs {
                    break;
                }
            }
            if res == self.lhs {
                return true;
            }
        }
        false
    }
}

fn parse_equations(file_path: &str) -> Result<Vec<Equation>, ()> {
    let raw =
        fs::read_to_string(file_path).map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    let mut result = vec![];
    for (line_idx, line) in raw
        .lines()
        .enumerate()
        .map(|(i, l)| (i, l.trim()))
        .filter(|(_, l)| l.len() > 0)
    {
        let splits = line
            .split(":")
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();
        if splits.len() != 2 {
            eprintln!(
                "{file_path}:{row}: ERROR: expected lhs: rhs",
                row = line_idx + 1
            );
            return Err(());
        }
        let lhs = splits[0].parse().map_err(|e| {
            eprintln!(
                "{file_path}:{row}: ERROR: failed to parse lhs: {e}",
                row = line_idx + 1
            );
        })?;
        let rhs = splits[1]
            .split(" ")
            .map(|s| s.trim())
            .filter(|&l| l.len() > 0)
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|e| {
                eprintln!(
                    "{file_path}:{row}: ERROR: Failed to parse rhs: {e}.",
                    row = line_idx + 1
                );
            })?;
        result.push(Equation { lhs, rhs });
    }
    Ok(result)
}

fn main() {
    let equations = parse_equations("input.txt").unwrap();
    let result_part1: u64 = equations
        .iter()
        .filter(|e| e.is_valid_part1())
        .map(|e| e.lhs)
        .sum();
    println!("Day 7, part 1: {result_part1}");
    let result_part2: u64 = equations
        .iter()
        .filter(|e| e.is_valid_part2())
        .map(|e| e.lhs)
        .sum();
    println!("Day 7, part 1: {result_part2}");
}
