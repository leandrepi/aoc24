use std::fs;

fn parse_input(file_path: &str) -> Result<Vec<Vec<u32>>, ()> {
    let data = fs::read_to_string(file_path).map_err(|e| eprintln!("Failed to read file: {e}"))?;
    let mut res: Vec<Vec<u32>> = vec![];
    for (line_idx, line) in data.lines().enumerate() {
        let splits: Vec<u32> = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|&l| !l.is_empty())
            .map(|s| s.parse())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| {
                eprintln!(
                    "{file_path}:{row}: Failed to parse line as a list of integers: {e}.",
                    row = line_idx + 1
                );
            })?;
        res.push(splits);
    }
    Ok(res)
}

fn is_safe_part1(xs: &[u32]) -> bool {
    if xs.len() == 1 {
        return true;
    }
    let increasing = xs[1] > xs[0];
    xs.iter().zip(xs[1..].iter()).all(|(&cur, &next)| {
        (next > cur) == increasing && cur != next && (cur as i32 - next as i32).abs() < 4
    })
}

fn is_safe_part2(xs: &[u32]) -> bool {
    if is_safe_part1(xs) {
        return true;
    }
    let mut ys: Vec<u32> = Vec::with_capacity(xs.len());
    for (x_idx, _x) in xs.iter().enumerate() {
        for (y_idx, &y) in xs.iter().enumerate() {
            if y_idx != x_idx {
                ys.push(y);
            }
        }
        if is_safe_part1(&ys) {
            return true;
        }
        ys.clear();
    }
    false
}

fn main() {
    let rows = parse_input("input.txt").unwrap();
    let part1: u32 = rows.iter().map(|xs| is_safe_part1(xs) as u32).sum();
    println!("Day 2, part 1: {part1}");
    let part2: u32 = rows.iter().map(|xs| is_safe_part2(xs) as u32).sum();
    println!("Day 2, part 2: {part2}");
}
