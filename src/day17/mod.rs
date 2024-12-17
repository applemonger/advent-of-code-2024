use aocd::*;
use cached::proc_macro::cached;
use regex::Regex;

fn read_program(input: &str) -> Vec<u8> {
    let re = Regex::new(r"Program: (.*)$").unwrap();
    let program = re.captures(input).unwrap().get(1).unwrap().as_str();
    program.split(',').map(|c| c.parse().unwrap()).collect()
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Machine {
    ptr: usize,
    a: isize,
    b: isize,
    c: isize,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"Register [A-Z]{1}: (\d+)").unwrap();
        let mut caps = re.captures_iter(value);
        let a: isize = caps.next().unwrap()[1].parse().unwrap();
        let b: isize = caps.next().unwrap()[1].parse().unwrap();
        let c: isize = caps.next().unwrap()[1].parse().unwrap();
        let ptr = 0;
        Machine { ptr, a, b, c }
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

    fn step(&mut self, opcode: u8, operand: u8) -> Option<isize> {
        let mut out = None;
        match opcode {
            0 => {
                self.a /= 2_isize.pow(self.combo(operand) as u32);
                self.ptr += 2;
            }
            1 => {
                self.b ^= self.combo(operand);
                self.ptr += 2;
            }
            2 => {
                self.b = self.combo(operand) % 8;
                self.ptr += 2;
            }
            3 => {
                if self.a != 0 {
                    self.ptr = operand as usize;
                } else {
                    self.ptr += 2;
                }
            }
            4 => {
                self.b ^= self.c;
                self.ptr += 2;
            }
            5 => {
                out = Some(self.combo(operand) % 8);
                self.ptr += 2;
            }
            6 => {
                self.b = self.a / 2_isize.pow(self.combo(operand) as u32);
                self.ptr += 2;
            }
            7 => {
                self.c = self.a / 2_isize.pow(self.combo(operand) as u32);
                self.ptr += 2;
            }
            _ => panic!("Invalid opcode."),
        }
        out
    }
}

#[cached]
fn process(mut state: Machine, program: Vec<u8>) -> Vec<isize> {
    let mut out = Vec::new();
    if state.ptr < program.len() {
        let opcode = program[state.ptr];
        let operand = program[state.ptr + 1];
        if let Some(o) = state.step(opcode, operand) {
            out.push(o);
        }
        out.extend(process(state, program));
    }
    out
}

fn read_out<T: ToString>(out: &Vec<T>) -> String {
    out.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[aocd(2024, 17)]
pub fn solution1() {
    let data = input!();
    let machine = Machine::from(data.as_str());
    let program = read_program(data.as_str());
    let out = process(machine, program);
    submit!(1, read_out(&out));
}

#[aocd(2024, 17)]
pub fn solution2() {}
