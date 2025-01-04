use std::fs;

fn parse_input() -> Result<(Vec<u64>, Vec<u8>), ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;
    let mut registers = vec![];
    let mut lines = raw
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty());

    for reg in ["A", "B", "C"] {
        let register: u64 = lines
            .next()
            .unwrap_or_else(|| panic!("should have reg {} line", reg))
            .split(format!("{}:", reg).as_str())
            .map(|l| l.trim())
            .nth(1)
            .unwrap_or_else(|| panic!("should have value for {}", reg))
            .parse()
            .map_err(|e| eprintln!("Failed to parse {reg} as int: {e}"))?;
        registers.push(register);
    }

    let program = lines
        .next()
        .expect("should have program line")
        .split("Program:")
        .map(|l| l.trim())
        .nth(1)
        .expect("Should have program opcodes")
        .split(",")
        .map(|c| c.parse())
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|e| eprintln!("Failed to parse program as list of opcodes: {e}"))?;

    Ok((registers, program))
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
}

impl Register {
    fn index(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
        }
    }

    fn value(&self, registers: &[u64]) -> u64 {
        registers[self.index()]
    }
}

#[derive(Debug)]
enum ComboOperand {
    Lit(u8),
    Reg(Register),
    Reserved,
}

impl ComboOperand {
    fn from(value: u8) -> Self {
        match value {
            0..=3 => Self::Lit(value),
            4 => Self::Reg(Register::A),
            5 => Self::Reg(Register::B),
            6 => Self::Reg(Register::C),
            7 => Self::Reserved,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl Instruction {
    fn from(opcode: u8) -> Self {
        match opcode {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            x => panic!("Invalid instruction opcode {x};"),
        }
    }

    fn apply(&self, operand: u8, registers: &mut [u64], ip: &mut usize, to_print: &mut Vec<u8>) {
        match self {
            Self::ADV | Self::BDV | Self::CDV => {
                let operand = ComboOperand::from(operand);
                let denom = 1
                    << (match operand {
                        ComboOperand::Lit(x) => x as u64,
                        ComboOperand::Reg(r) => r.value(registers),
                        x => panic!("Invalid operand {x:?} for a xdv instruction."),
                    });
                let num = Register::A.value(registers);
                let store_register = match self {
                    Self::ADV => Register::A,
                    Self::BDV => Register::B,
                    Self::CDV => Register::C,
                    _ => unreachable!(),
                }
                .index();
                registers[store_register] = num / denom;
            }
            Self::BXL => registers[Register::B.index()] ^= operand as u64,
            Self::BST => {
                let operand = ComboOperand::from(operand);
                registers[Register::B.index()] = match operand {
                    ComboOperand::Lit(x) => x as u64,
                    ComboOperand::Reg(r) => r.value(registers),
                    x => panic!("Invalid operand {x:?} for bst instruction."),
                } % 8;
            }
            Self::JNZ => {
                if Register::A.value(registers) != 0 {
                    *ip = operand as usize;
                    return;
                }
            }
            Self::BXC => registers[Register::B.index()] ^= Register::C.value(registers),
            Self::OUT => {
                let operand = ComboOperand::from(operand);
                let value = match operand {
                    ComboOperand::Lit(x) => x as u64,
                    ComboOperand::Reg(r) => r.value(registers),
                    x => panic!("Invalid operand {x:?} for out instruction."),
                } % 8;
                to_print.push(value as u8);
            }
        }
        *ip += 2;
    }
}

fn run_program(registers: &mut [u64], program: &[u8], print: bool) -> Vec<u8> {
    let mut ip = 0;
    let mut to_print = vec![];

    while ip < program.len() - 1 {
        let instruction = Instruction::from(program[ip]);
        instruction.apply(program[ip + 1], registers, &mut ip, &mut to_print);
    }

    if print {
        print_output(&to_print)
    }
    to_print
}

fn print_output(to_print: &[u8]) {
    if to_print.is_empty() {
        println!();
        return;
    }
    print!("{}", to_print[0]);
    for c in to_print[1..].iter() {
        print!(",{c}")
    }
    println!()
}

fn backtrack_a(program: &[u8], b_idx: usize, a_min: u64) -> Option<u64> {
    // reading the program we see that's it basically the following:
    // do {
    //     B = A % 8;
    //     ... // various operations on B and C, without writing to A nor stdout
    //     A = A / 8;
    //     printf("%d,", B % 8);
    // }
    // while (A != 0)
    //
    // Therefore we know that B may only take at most 8 values at the beginning of
    // the block, and we know its value at the end of the block, because the program
    // has to print itself. We also know that A has to be 0 for the program to end;
    // since it's only divided by 8 on each iteration, we can pin its value between 1
    // and 8 (excluded) for the last iteration. We brute force all values of A mod 8
    // for B to match the last printed char and find at most one or two candidates for
    // A (in my case only 4), which allows to us to backtrack to the previous opcode,
    // where we repeat the same opeation (trying for A values ranging from 32 to 40).
    let b_opcode = program[b_idx];
    for b in 0..8 {
        let a = a_min + b as u64;

        let prog_without_jump = run_program(&mut [a, 0, 0], &program[..(program.len() - 2)], false);
        assert!(
            prog_without_jump.len() == 1,
            "Program only prints once per loop."
        );
        let result = prog_without_jump[0];

        if result == b_opcode {
            if b_idx == 0 {
                // reached and matched the firsst opcode: quine!
                return Some(a);
            } else if let Some(r) = backtrack_a(program, b_idx - 1, a * 8) {
                return Some(r);
            }
        }
    }
    None
}

fn find_quine(program: &[u8]) -> Option<u64> {
    backtrack_a(program, program.len() - 1, 0)
}

fn main() {
    let (mut registers, program) = parse_input().unwrap();
    print!("Day 17, part 1: ");
    run_program(&mut registers, &program, true);
    let snd = find_quine(&program).expect("ERROR: Could not find quine for program");
    run_program(&mut [snd, 0, 0], &program, true);
    println!("Day 17, part 2: {snd}");
}
