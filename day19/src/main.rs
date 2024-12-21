use std::fs;

pub struct Towels {
    patterns: Vec<Node>,
    designs: Vec<String>,
}

impl Towels {
    pub fn walk_trie(&self) -> (usize, usize) {
        let (mut part1, mut part2) = (0, 0);
        for design in &self.designs {
            let d_len = design.len();
            let mut valid_ways = vec![0; d_len + 1];
            valid_ways[0] = 1;
            for start in 0..d_len {
                if valid_ways[start] != 0 {
                    let mut i = 0;
                    for end in start..d_len {
                        i = self.patterns[i].next[char_to_index(design.as_bytes()[end])];
                        if i == 0 {
                            break;
                        }
                        if self.patterns[i].valid {
                            valid_ways[end + 1] += valid_ways[start];
                        }
                    }
                }
            }
            let total_ways = valid_ways[d_len];
            part1 += (total_ways > 0) as usize;
            part2 += total_ways;
        }
        (part1, part2)
    }
}

fn parse_input() -> Result<Towels, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    let mut lines = raw.lines().map(|l| l.trim()).filter(|l| l.len() > 0);

    let mut pattern_trie = Vec::with_capacity(1000);
    pattern_trie.push(Node::new()); // root node
    for pattern in lines
        .next()
        .expect("Pattern line")
        .split(",")
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
    {
        let mut i = 0;
        for j in pattern.bytes().map(char_to_index) {
            if pattern_trie[i].next[j] == 0 {
                pattern_trie[i].next[j] = pattern_trie.len();
                pattern_trie.push(Node::new())
            }
            i = pattern_trie[i].next[j];
        }
        pattern_trie[i].valid = true;
    }
    let designs = lines.map(|s| s.to_owned()).collect();
    Ok(Towels {
        patterns: pattern_trie,
        designs,
    })
}

pub struct Node {
    next: [usize; 5],
    valid: bool,
}

fn char_to_index(c: u8) -> usize {
    match c as char {
        'u' => 0,
        'b' => 1,
        'w' => 2,
        'r' => 3,
        'g' => 4,
        _ => unreachable!(),
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            next: [0; 5],
            valid: false,
        }
    }
}

fn main() {
    let towels = parse_input().unwrap();
    let (fst, snd) = towels.walk_trie();
    println!("Day 19, part 1: {}", fst);
    println!("Day 19, part 2: {}", snd);
}
