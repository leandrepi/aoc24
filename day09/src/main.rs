use std::fs;

fn parse_input(filepath: &str) -> Result<Vec<u8>, ()> {
    let data =
        fs::read_to_string(filepath).map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    Ok(data
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).expect("digit") as u8)
        .collect())
}

fn to_blocks(disk_map: &[u8]) -> Vec<i64> {
    let mut res = vec![];
    for (idx, c) in disk_map.iter().enumerate() {
        let to_push = match idx % 2 {
            0 => idx as i64 / 2,
            1 => -1,
            _ => unreachable!(),
        };
        for _ in 0..*c {
            res.push(to_push);
        }
    }
    res
}

fn compact_part1(blocks: &mut [i64]) {
    let mut fst_empty = 0;
    for i in 0..blocks.len() {
        let cursor = blocks.len() - 1 - i;
        if fst_empty >= cursor {
            return;
        }
        if blocks[cursor] < 0 {
            continue;
        }
        if let Some(idx) = blocks[fst_empty..]
            .iter()
            .enumerate()
            .filter(|(_, &c)| c < 0)
            .map(|(idx, _)| idx)
            .next()
        {
            fst_empty += idx;
        } else {
            // no free space, nothing to do
            return;
        }
        blocks.swap(cursor, fst_empty);
        fst_empty += 1;
    }
}

fn compact_part2(blocks: &mut [i64]) {
    let mut cursor = blocks.len() - 1;
    let mut last_contiguous = 0;
    loop {
        if blocks[cursor] < 0 {
            cursor -= 1;
            continue;
        }
        let block_value = blocks[cursor];
        let mut block_size = 0;
        while blocks[cursor] == block_value {
            block_size += 1;
            if cursor == 0 {
                return;
            }
            cursor -= 1;
        }
        cursor += 1;
        let mut fst_empty = last_contiguous;
        let mut non_contig = false;
        loop {
            while fst_empty < cursor && blocks[fst_empty] >= 0 {
                fst_empty += 1;
            }
            if !non_contig {
                last_contiguous = fst_empty;
                non_contig = true;
                if last_contiguous >= cursor {
                    return;
                }
            }
            if fst_empty >= cursor {
                break;
            }
            let mut size_empty = 0;
            while blocks[fst_empty + size_empty] < 0 {
                size_empty += 1;
            }
            if size_empty >= block_size {
                for k in 0..block_size {
                    blocks.swap(cursor + k, fst_empty + k)
                }
                break;
            }
            fst_empty += size_empty;
        }
        cursor -= 1;
    }
}

fn blocks_to_result(blocks: &[i64]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, &c)| c >= 0)
        .map(|(i, &c)| i as u64 * c as u64)
        .sum()
}

fn main() {
    let input = parse_input("input.txt").unwrap();
    let mut blocks = to_blocks(&input);
    let mut blocks1 = blocks.clone();
    compact_part1(&mut blocks1);
    let fst = blocks_to_result(&blocks1);
    println!("Day 9, part 1: {fst}");
    compact_part2(&mut blocks);
    let snd = blocks_to_result(&blocks);
    println!("Day 9, part 2: {snd}");
}
