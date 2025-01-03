use aocd::*;
use regex::Regex;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Machine {
    ptr: usize,
    a: isize,
    b: isize,
    c: isize,
    program: Vec<u8>,
    out: Vec<u8>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"Register [A-Z]{1}: (\d+)").unwrap();
        let mut caps = re.captures_iter(value);
        let a: isize = caps.next().unwrap()[1].parse().unwrap();
        let b: isize = caps.next().unwrap()[1].parse().unwrap();
        let c: isize = caps.next().unwrap()[1].parse().unwrap();
        let re = Regex::new(r"Program: (.*)$").unwrap();
        let program = re.captures(value).unwrap().get(1).unwrap().as_str();
        let program = program.split(',').map(|c| c.parse().unwrap()).collect();
        Machine {
            ptr: 0,
            a,
            b,
            c,
            program,
            out: Vec::new(),
        }
    }
}

impl Machine {
    fn combo(&self, operand: u8) -> isize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Reserved operand."),
            _ => panic!("Unkown operand."),
        }
    }

    fn execute(&mut self) {
        let opcode = self.program[self.ptr];
        let operand = self.program[self.ptr + 1];
        let mut jump = 2;
        match opcode {
            0 => self.a /= 2_isize.pow(self.combo(operand) as u32),
            1 => self.b ^= self.combo(operand),
            2 => self.b = self.combo(operand) % 8,
            3 => {
                if self.a != 0 {
                    self.ptr = operand as usize;
                    jump = 0;
                }
            }
            4 => self.b ^= self.c,
            5 => self.out.push((self.combo(operand) % 8) as u8),
            6 => self.b = self.a / 2_isize.pow(self.combo(operand) as u32),
            7 => self.c = self.a / 2_isize.pow(self.combo(operand) as u32),
            _ => panic!("Invalid opcode."),
        }
        self.ptr += jump;
        if self.ptr < self.program.len() {
            self.execute();
        }
    }
}

#[aocd(2024, 17)]
pub fn solution1() {
    let data = input!();
    let mut machine = Machine::from(data.as_str());
    machine.execute();
    let out: Vec<String> = machine.out.iter().map(|s| s.to_string()).collect();
    submit!(1, out.join(","));
}

/// Adapted from https://github.com/Praful/advent_of_code/blob/main/2024/src/day17.py
fn solve(a: isize, idx: usize, possible: &mut Vec<isize>, default_machine: &Machine) {
    for n in 0..8 {
        let mut machine = default_machine.clone();
        let candidate = (a << 3) | n;
        machine.a = candidate;
        machine.execute();
        if machine.out == machine.program[(machine.program.len() - idx)..] {
            if machine.out == machine.program {
                possible.push(candidate);
            } else {
                solve(candidate, idx + 1, possible, default_machine);
            }
        }
    }
}

#[aocd(2024, 17)]
pub fn solution2() {
    let data = input!();
    let machine = Machine::from(data.as_str());
    let mut possible = Vec::new();
    solve(0, 1, &mut possible, &machine);
    let best = possible.iter().min().unwrap();
    submit!(2, *best);
}
