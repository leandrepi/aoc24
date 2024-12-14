use std::fs;

const A_TOKENS: u64 = 3;
const B_TOKENS: u64 = 1;
const PART2_PRIZE_OFFSET: u64 = 10000000000000;

#[derive(Debug)]
pub struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

fn parse_xy(split: &str) -> Result<(u64, u64), ()> {
    let splits = split
        .split(",")
        .map(|l| {
            l.trim()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
        })
        .map(|s| s.parse())
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|e| eprintln!("Failed to parse int: {e}"))?;
    if splits.len() != 2 {
        eprintln!("Invalid X, Y section.");
        return Err(());
    }
    Ok((splits[0], splits[1]))
}

fn parse_input() -> Result<Vec<Machine>, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    let mut machines = vec![];
    let mut lines = raw.lines().map(|l| l.trim()).filter(|l| l.len() > 0);
    while let Some(line) = lines.next() {
        let button_a = line
            .split("A:")
            .map(|l| l.trim())
            .nth(1)
            .expect("should have xy for A");
        let a = parse_xy(button_a)?;
        let button_b = lines
            .next()
            .expect("Should have button B")
            .split("B:")
            .map(|l| l.trim())
            .nth(1)
            .expect("should have xy for B");
        let b = parse_xy(button_b)?;
        let prize = lines
            .next()
            .expect("Should have prize")
            .split("Prize:")
            .map(|l| l.trim())
            .nth(1)
            .expect("should have xy for B");
        let prize = parse_xy(prize)?;
        machines.push(Machine { a, b, prize });
    }
    Ok(machines)
}

fn update_machines_part2(machines: &mut [Machine]) {
    for machine in machines.iter_mut() {
        (*machine).prize = (
            (*machine).prize.0 + PART2_PRIZE_OFFSET,
            (*machine).prize.1 + PART2_PRIZE_OFFSET,
        );
    }
}

fn gcd_ext(a: u64, b: u64) -> (u64, i64, i64) {
    let (mut prev_r, mut r) = (a, b);
    let (mut prev_s, mut s) = (1, 0);
    let (mut prev_t, mut t) = (0, 1);

    while r != 0 {
        let q = prev_r / r;
        (prev_r, r) = (r, prev_r - q * r);
        (prev_s, s) = (s, prev_s - (q as i64) * s);
        (prev_t, t) = (t, prev_t - (q as i64) * t);
    }
    (prev_r, prev_s, prev_t)
}

fn det2(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}

fn diophantine_solution(a: u64, b: u64, c: u64) -> Option<(i64, i64, u64)> {
    let (gcd, mut u, mut v) = gcd_ext(a, b);
    if c % gcd != 0 {
        return None;
    }
    let h = (c / gcd) as i64;
    u *= h;
    v *= h;
    Some((u, v, gcd))
}

fn minimal_token_cost(machine: &Machine) -> Option<u64> {
    let (px, py) = machine.prize;
    let (ax, ay) = machine.a;
    let (bx, by) = machine.b;

    // find some solution for both Diophantine equations
    let (ux, vx, gcdx) = diophantine_solution(ax, bx, px)?;
    let (uy, vy, gcdy) = diophantine_solution(ay, by, py)?;

    // we'll need the same tokens to achieve both x and y totals
    // therefore we get a 2x2 system of equations with two unknowns
    // solving it with Cramer's rule
    let (c1, c2) = (uy - ux, vy - vx);
    let (a1, a2) = ((bx / gcdx) as i64, -((ax / gcdx) as i64));
    let (b1, b2) = (-((by / gcdy) as i64), (ay / gcdy) as i64);

    let denom = det2(a1, b1, a2, b2);
    let num = det2(c1, b1, c2, b2);
    if denom == 0 || num % denom != 0 {
        return None;
    }
    let k = num / denom;

    // convert back to an amount of tokens
    let a = ux + k * ((bx / gcdx) as i64);
    let b = vx - k * ((ax / gcdx) as i64);
    let tokens = (a as u64) * A_TOKENS + (b as u64) * B_TOKENS;
    Some(tokens)
}

fn main() {
    let mut machines = parse_input().unwrap();
    let fst: u64 = machines.iter().filter_map(|m| minimal_token_cost(m)).sum();
    println!("Day 13, part 1: {fst}");
    update_machines_part2(&mut machines);
    let snd: u64 = machines.iter().filter_map(|m| minimal_token_cost(m)).sum();
    println!("Day 13, part 2: {snd}");
}
