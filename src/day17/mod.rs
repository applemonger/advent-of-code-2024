use aocd::*;
use regex::Regex;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct Machine {
    ptr: usize,
    a: isize,
    b: isize,
    c: isize,
    program: Vec<u8>,
    initial: (isize, isize, isize),
    out: Vec<u8>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"Register [A-Z]{1}: (\d+)").unwrap();
        let mut caps = re.captures_iter(value);
        let a: isize = caps.next().unwrap()[1].parse().unwrap();
        let b: isize = caps.next().unwrap()[1].parse().unwrap();
        let c: isize = caps.next().unwrap()[1].parse().unwrap();
        let ptr = 0;
        let re = Regex::new(r"Program: (.*)$").unwrap();
        let program = re.captures(value).unwrap().get(1).unwrap().as_str();
        let program = program.split(',').map(|c| c.parse().unwrap()).collect();
        let initial = (a, b, c);
        let out = Vec::new();
        Machine {
            ptr,
            a,
            b,
            c,
            program,
            initial,
            out,
        }
    }
}

impl Machine {
    fn reset(&mut self) {
        self.ptr = 0;
        self.a = self.initial.0;
        self.b = self.initial.1;
        self.c = self.initial.2;
        self.out = Vec::new();
    }

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
                self.out.push((self.combo(operand) % 8) as u8);
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
        if self.ptr < self.program.len() {
            self.execute();
        }
    }
}

fn read_out<T: ToString>(out: &[T]) -> String {
    out.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[aocd(2024, 17)]
pub fn solution1() {
    let data = input!();
    let mut machine = Machine::from(data.as_str());
    machine.execute();
    submit!(1, read_out(&machine.out));
}

#[aocd(2024, 17)]
pub fn solution2() {
    let data = input!();
    let mut machine = Machine::from(data.as_str());
    let mut a = 2_isize.pow(3 * 16);
    'search: loop {
        a -= 1;
        machine.reset();
        machine.a = a;
        machine.execute();
        if machine.out == machine.program {
            break 'search;
        }
    }
    submit!(1, a);
}
