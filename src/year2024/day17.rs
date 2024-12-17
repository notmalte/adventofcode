use itertools::Itertools;

use crate::{malformed, Answer};

pub fn run(input: &str) -> Answer {
    Answer {
        part1: part1(input),
        part2: part2(input),
    }
}

fn part1(input: &str) -> String {
    let (rs, ins, _) = parse_input(input);

    Simulator::new(ins).run(rs).iter().join(",")
}

fn part2(input: &str) -> String {
    // this is mostly the result of manual analysis, so
    // it might not work for all possible puzzle inputs

    let (_, ins, ins_u8) = parse_input(input);

    let sim = Simulator::new(ins);

    let length = ins_u8.len();

    let mut candidates = Vec::new();
    let mut solutions = Vec::new();

    for n in 0..8 {
        candidates.push(vec![n]);
    }

    while let Some(candidate) = candidates.pop() {
        let mut av = candidate.clone();
        av.resize(length, 1);
        let a = av.iter().fold(0, |acc, &digit| acc * 8 + digit);

        let rs = RegisterState { a, b: 0, c: 0 };
        let out = sim.run(rs);

        if out.len() != length {
            continue;
        }

        if out == ins_u8 {
            solutions.push(a);
            continue;
        }

        let i = length - candidate.len();

        let digit_want = ins_u8[i];
        let digit_found = out[i];

        if digit_found == digit_want {
            for n in 0..8 {
                let mut new_candidate = candidate.clone();
                new_candidate.push(n);
                candidates.push(new_candidate);
            }
        }
    }

    if solutions.is_empty() {
        malformed("2024", "17");
    }

    solutions.iter().min().unwrap().to_string()
}

struct Simulator {
    ins: Vec<Instruction>,
}

impl Simulator {
    fn new(ins: Vec<Instruction>) -> Self {
        Self { ins }
    }

    fn run(&self, mut rs: RegisterState) -> Vec<u8> {
        let mut ic = 0;
        let mut out = Vec::new();

        while ic < self.ins.len() {
            let i = self.ins[ic];

            match i {
                Instruction::Adv(co) => {
                    rs.a /= 2usize.pow(co.resolve(&rs) as u32);
                }
                Instruction::Bxl(n) => {
                    rs.b ^= n as usize;
                }
                Instruction::Bst(co) => {
                    rs.b = co.resolve(&rs) % 8;
                }
                Instruction::Jnz(n) => {
                    if rs.a != 0 {
                        ic = (n as usize) / 2;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    rs.b ^= rs.c;
                }
                Instruction::Out(co) => {
                    out.push((co.resolve(&rs) % 8) as u8);
                }
                Instruction::Bdv(co) => {
                    rs.b = rs.a / 2usize.pow(co.resolve(&rs) as u32);
                }
                Instruction::Cdv(co) => {
                    rs.c = rs.a / 2usize.pow(co.resolve(&rs) as u32);
                }
            }

            ic += 1;
        }

        out
    }
}

fn parse_input(input: &str) -> (RegisterState, Vec<Instruction>, Vec<u8>) {
    let (registers_s, program_s) = input
        .trim()
        .split_once("\n\n")
        .unwrap_or_else(|| malformed("2024", "17"));

    let registers_v: Vec<_> = registers_s
        .lines()
        .map(|l| {
            l.split_once(": ")
                .unwrap_or_else(|| malformed("2024", "17"))
                .1
                .parse()
                .unwrap_or_else(|_| malformed("2024", "17"))
        })
        .collect();

    if registers_v.len() != 3 {
        malformed("2024", "17");
    }

    let registers = RegisterState {
        a: registers_v[0],
        b: registers_v[1],
        c: registers_v[2],
    };

    let instructions_iter = program_s
        .split_once(": ")
        .unwrap_or_else(|| malformed("2024", "17"))
        .1
        .split(",")
        .map(|n| n.parse().unwrap_or_else(|_| malformed("2024", "17")));

    let instructions_u8: Vec<_> = instructions_iter.clone().collect();

    let instructions = instructions_iter
        .tuples()
        .map(|(opcode, operand)| {
            if operand > 7 {
                malformed("2024", "17");
            }

            match opcode {
                0 => ComboOperand::from(operand).map(Instruction::Adv),
                1 => Some(Instruction::Bxl(operand)),
                2 => ComboOperand::from(operand).map(Instruction::Bst),
                3 => (operand % 2 == 0).then_some(Instruction::Jnz(operand)),
                4 => Some(Instruction::Bxc),
                5 => ComboOperand::from(operand).map(Instruction::Out),
                6 => ComboOperand::from(operand).map(Instruction::Bdv),
                7 => ComboOperand::from(operand).map(Instruction::Cdv),
                _ => None,
            }
            .unwrap_or_else(|| malformed("2024", "17"))
        })
        .collect();

    (registers, instructions, instructions_u8)
}

#[derive(Debug, Clone)]
struct RegisterState {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(u8),
    Bst(ComboOperand),
    Jnz(u8),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComboOperand {
    Literal(u8),
    Register(Register),
}

impl ComboOperand {
    fn from(n: u8) -> Option<Self> {
        match n {
            0..=3 => Some(Self::Literal(n)),
            4 => Some(Self::Register(Register::A)),
            5 => Some(Self::Register(Register::B)),
            6 => Some(Self::Register(Register::C)),
            _ => None,
        }
    }

    fn resolve(&self, rs: &RegisterState) -> usize {
        match self {
            Self::Literal(n) => *n as usize,
            ComboOperand::Register(r) => match r {
                Register::A => rs.a,
                Register::B => rs.b,
                Register::C => rs.c,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
    C,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            ),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }
}
