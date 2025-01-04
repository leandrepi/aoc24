use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Pages {
    rule_map: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl Pages {
    fn valid_invalid(&self) -> (u32, u32) {
        let mut valid_sum = 0;
        let mut invalid_sum = 0;
        for update in &self.updates {
            let mut visited = vec![];
            let mut invalid = false;
            for item in update {
                let dependencies = self.rule_map.get(item).expect("Page should be in map");
                let mut tail_idx = visited.len();
                visited.push(item);
                if let Some((idx, _vf)) = visited
                    .iter()
                    .enumerate()
                    .find(|&(_, v)| dependencies.contains(v))
                {
                    while tail_idx > idx {
                        visited.swap(tail_idx, tail_idx - 1);
                        tail_idx -= 1;
                    }
                    invalid = true;
                }
            }
            if invalid {
                invalid_sum += visited[visited.len() / 2];
            } else {
                valid_sum += update[update.len() / 2];
            }
        }
        (valid_sum, invalid_sum)
    }
}

fn parse_pages(content: &str) -> Result<Pages, ()> {
    let mut lines = content.lines().map(|l| l.trim()).enumerate();
    let mut rule_map = HashMap::new();
    for (idx, rule_line) in lines.by_ref() {
        if rule_line.is_empty() {
            break;
        }
        let splits: Vec<u32> = rule_line
            .split("|")
            .map(|s| s.trim())
            .filter(|&l| !l.is_empty())
            .map(|s| s.parse())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| {
                eprintln!(
                    "ERROR:  Failed to parse line {row} as an integer rule: {e}.",
                    row = idx + 1
                );
            })?;
        let &[fst, snd] = splits.as_slice() else {
            eprintln!(
                "ERROR: {row}: Invalid rule line, found {_len} values instead of 2.",
                row = idx + 1,
                _len = splits.len()
            );
            return Err(());
        };
        rule_map
            .entry(fst)
            .and_modify(|c: &mut Vec<u32>| c.push(snd))
            .or_insert(vec![snd]);
        rule_map.entry(snd).or_insert(vec![]);
    }
    let mut updates = vec![];
    for (idx, update_line) in lines.filter(|(_, l)| !l.is_empty()) {
        let update: Vec<u32> = update_line
            .split(",")
            .map(|s| s.trim())
            .filter(|&l| !l.is_empty())
            .map(|s| s.parse())
            .collect::<Result<Vec<u32>, _>>()
            .map_err(|e| {
                eprintln!(
                    "ERROR:  Failed to parse line {row} as an list of page updates: {e}.",
                    row = idx + 1
                );
            })?;
        updates.push(update);
    }
    Ok(Pages { rule_map, updates })
}

fn main() {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let pages = parse_pages(&raw).unwrap();
    let (result_part1, result_part2) = pages.valid_invalid();
    println!("Day 5, part 1: {result_part1}");
    println!("Day 5, part 2: {result_part2}");
}
