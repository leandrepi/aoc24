use std::{collections::HashMap, fs};
type Wire = (u8, u8, u8);
const OUTPUT_WIRE: u8 = 'z' as u8;
const X_INPUT_WIRE: u8 = 'x' as u8;
const Y_INPUT_WIRE: u8 = 'y' as u8;
const INPUT_N_BITS: usize = 45;

#[derive(Debug, PartialEq, Clone)]
pub enum Gate {
    XOR,
    AND,
    OR,
}

impl Gate {
    pub fn from(raw: &str) -> Result<Self, ()> {
        Ok(match raw {
            "XOR" => Self::XOR,
            "AND" => Self::AND,
            "OR" => Self::OR,
            x => {
                eprintln!("Invalid gate {x}, expected XOR, AND or OR.");
                return Err(());
            }
        })
    }
}

fn parse_wire(raw: &str) -> Result<Wire, ()> {
    let bytes = raw.as_bytes();
    if bytes.len() != 3 {
        eprintln!("Invalid wire, should be xyz.");
        return Err(());
    }
    Ok((bytes[0], bytes[1], bytes[2]))
}

fn parse_init_wire(raw: &str) -> Result<(Wire, bool), ()> {
    let splits = raw
        .split(':')
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>();
    if splits.len() != 2 {
        eprintln!("Invalid wire initialisation, should be w: 1.");
        return Err(());
    }
    let value: u8 = splits[1]
        .parse()
        .map_err(|e| eprintln!("Failed to parse wire value as int: {e}."))?;
    if value > 1 {
        eprintln!("Wire value {value} is not a boolean");
        return Err(());
    }
    let wire = parse_wire(&splits[0])?;
    Ok((wire, value != 0))
}

#[derive(Debug, Clone)]
pub struct Connection {
    l: Wire,
    r: Wire,
    gate: Gate,
    o: Wire,
}

impl Connection {
    pub fn from(line: &str) -> Result<Self, ()> {
        let splits = line
            .split("->")
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();
        if splits.len() != 2 {
            eprintln!("Invalid line, should be x GATE y -> z.");
            return Err(());
        }
        let o = parse_wire(&splits[1])?;
        let input = splits[0]
            .split(' ')
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();
        if input.len() != 3 {
            eprintln!("Invalid connection input, should be x GATE y.");
            return Err(());
        }
        let gate = Gate::from(&input[1])?;
        let l = parse_wire(&input[0])?;
        let r = parse_wire(&input[2])?;
        Ok(Self { l, r, o, gate })
    }

    fn input_matches(&self, input: Wire, gate: &Gate) -> bool {
        (self.l == input || self.r == input) && self.gate == *gate
    }
}

pub struct GateSystem {
    wires: HashMap<Wire, bool>,
    connections: Vec<Connection>,
}

impl GateSystem {
    fn run(&mut self) -> u64 {
        let mut seen = vec![false; self.connections.len()];
        let mut n_seen = 0;

        // z wires aren't part of the input, so we increment the total on the fly
        let mut total_output = 0;
        loop {
            if n_seen == self.connections.len() {
                break;
            }
            for (idx, c) in self.connections.iter().enumerate() {
                if seen[idx] {
                    continue;
                }
                let lhs = self.wires.get(&c.l);
                if lhs == None {
                    continue;
                }
                let rhs = self.wires.get(&c.r);
                if rhs == None {
                    continue;
                }
                let lhs = lhs.unwrap();
                let rhs = rhs.unwrap();
                let res = match c.gate {
                    Gate::AND => lhs & rhs,
                    Gate::OR => lhs | rhs,
                    Gate::XOR => lhs ^ rhs,
                };
                self.wires.insert(c.o, res);
                seen[idx] = true;
                n_seen += 1;
                if res && c.o.0 == OUTPUT_WIRE {
                    total_output += 1 << wire_to_shift(c.o);
                }
            }
        }
        total_output
    }

    fn connection_graph(&self) -> HashMap<Wire, Vec<usize>> {
        let mut res = HashMap::new();
        for (idx, c) in self.connections.iter().enumerate() {
            res.entry(c.l)
                .and_modify(|c: &mut Vec<_>| c.push(idx))
                .or_insert(vec![idx]);
            res.entry(c.r)
                .and_modify(|c| c.push(idx))
                .or_insert(vec![idx]);
        }
        res
    }

    fn find_swaps(&mut self) -> Vec<Wire> {
        let graph = self.connection_graph();
        let mut carry = None;
        let mut swaps = vec![];
        for cur in 0..INPUT_N_BITS {
            let (c, swap) = find_next_carry_and_swap(&graph, &mut self.connections, cur, carry);
            carry = Some(c);
            if let Some((o1, o2)) = swap {
                swaps.push(o1);
                swaps.push(o2);
            }
        }
        swaps
    }
}

fn find_next(
    graph: &HashMap<Wire, Vec<usize>>,
    connections: &[Connection],
    from: Wire,
    with: Wire,
    gate: Gate,
) -> Option<(usize, Wire)> {
    graph
        .get(&from)
        .unwrap()
        .iter()
        .map(|&i| (i, &connections[i]))
        .filter(|(_, c)| c.input_matches(with, &gate))
        .map(|(i, c)| (i, c.o))
        .next()
}

fn find_next_carry_and_swap(
    graph: &HashMap<Wire, Vec<usize>>,
    connections: &mut [Connection],
    cur: usize,
    carry: Option<Wire>,
) -> (Wire, Option<(Wire, Wire)>) {
    // swaps on the fly based on improper carries
    // the basic addition scheme is always as follows (when no swaps)
    // assuming a previous carry c_{k-1}
    // x_k XOR y_k -> next_k
    // x_k AND y_k -> next_carry_k
    // next_k AND carry{k-1} -> next_with_carry_k
    // next_k XOR carry{k-1} -> z_k
    // next_with_carry_k OR next_carry_k -> carry_k

    // The code is messy but I can't be bothered with prettier checks
    let (w1, w2) = shift_to_wire_num(cur as u8);
    let x = (X_INPUT_WIRE, w1, w2);
    let y = (Y_INPUT_WIRE, w1, w2);
    let z = (OUTPUT_WIRE, w1, w2);
    let (n_idx, mut next) = find_next(graph, connections, x, y, Gate::XOR).unwrap();
    let (nc_idx, mut next_carry) = find_next(graph, connections, x, y, Gate::AND).unwrap();
    let mut swap = None;
    if let Some(car) = carry {
        let mut with_carry = find_next(graph, connections, car, next, Gate::AND);
        if with_carry == None {
            connections[n_idx].o = next_carry;
            connections[nc_idx].o = next;
            swap = Some((next_carry, next));
            next = next_carry;
            next_carry = connections[nc_idx].o;
            with_carry = find_next(graph, connections, car, next, Gate::AND);
        }
        let (nwc, mut next_with_carry) = with_carry.unwrap();
        let (nn, new_next) = find_next(graph, connections, next, car, Gate::XOR).unwrap();
        if new_next != z {
            if next_carry == z {
                connections[nn].o = next_carry;
                connections[nc_idx].o = new_next;
                swap = Some((next_carry, new_next));
                next_carry = new_next;
            } else {
                if next_with_carry == z {
                    connections[nn].o = next_with_carry;
                    connections[nwc].o = new_next;
                    swap = Some((next_with_carry, new_next));
                    next_with_carry = new_next;
                }
            }
        }
        let (nc, mut final_carry) =
            find_next(graph, connections, next_with_carry, next_carry, Gate::OR).unwrap();
        if new_next != z && final_carry == z {
            connections[nn].o = final_carry;
            connections[nc].o = new_next;
            swap = Some((final_carry, new_next));
            final_carry = new_next;
        }
        next_carry = final_carry;
    }
    (next_carry, swap)
}

fn wire_to_shift(w: Wire) -> u8 {
    let bot = '0' as u8;
    let top = '9' as u8;
    if w.1 < bot || w.1 > top || w.2 < bot || w.2 > top {
        panic!("Invalid wire shift, should have 2 ascii digits.")
    }
    (w.1 - bot) * 10 + w.2 - bot
}

fn shift_to_wire_num(shift: u8) -> (u8, u8) {
    ('0' as u8 + shift / 10, '0' as u8 + shift % 10)
}

fn swaps_to_answer(swaps: &[Wire]) -> String {
    let mut swap_strings: Vec<String> = swaps
        .iter()
        .map(|&(a, b, c)| format!("{}{}{}", a as char, b as char, c as char))
        .collect();
    swap_strings.sort();
    swap_strings.join(",")
}

fn parse_input() -> Result<GateSystem, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;

    let parts = raw
        .trim()
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    if parts.len() != 2 {
        eprintln!(
            "Invalid input, expected 2 sections separated by an empty line, got {}.",
            parts.len()
        );
        return Err(());
    }
    let init = parts[0]
        .lines()
        .map(|l| parse_init_wire(l.trim()))
        .collect::<Result<_, _>>()?;
    let connections = parts[1]
        .lines()
        .map(|l| Connection::from(l.trim()))
        .collect::<Result<_, _>>()?;
    Ok(GateSystem {
        wires: init,
        connections,
    })
}

fn main() {
    let mut system = parse_input().unwrap();
    let fst = system.run();
    println!("Day 24, part 1: {fst}");
    let swap_fixes = system.find_swaps();
    let snd = swaps_to_answer(&swap_fixes);
    println!("Day 24, part 2: {snd}");
}
