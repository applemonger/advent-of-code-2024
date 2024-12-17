use std::collections::HashMap;

use aocd::*;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Default)]
struct Machine {
    registers: HashMap<char, isize>,
    program: Vec<u8>,
    out: Vec<isize>
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"Register ([A-Z]{1}): (\d+)").unwrap();
        let mut registers = HashMap::new();
        for capture in re.captures_iter(value) {
            let register = capture.get(1).unwrap().as_str().chars().next().unwrap();
            let value: isize = capture.get(2).unwrap().as_str().parse().unwrap();
            registers.insert(register, value);
        }
        let re = Regex::new(r"Program: (.*)$").unwrap();
        let program = re.captures(value).unwrap().get(1).unwrap().as_str();
        let program: Vec<u8> = program.split(',').map(|c| c.parse().unwrap()).collect();
        let out = Vec::new();
        Machine { registers, program, out }
    }
}

impl Machine {
    fn reg(&self, register: char) -> isize {
        *self.registers.get(&register).unwrap()
    }

    fn set(&mut self, register: char, value: isize) {
        self.registers.entry(register).and_modify(|x| *x = value);
    }

    fn combo(&self, operand: u8) -> isize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg('A'),
            5 => self.reg('B'),
            6 => self.reg('C'),
            7 => panic!("Reserved operand."),
            _ => panic!("Unkown operand.")
        }
    }

    fn process(&mut self, mut ptr: usize) {
        let opcode = self.program[ptr];
        let operand = self.program[ptr+1];
        match opcode {
            0 => {
                let value = self.reg('A') / 2_isize.pow(self.combo(operand) as u32);
                self.set('A', value);
                ptr += 2;
            }
            1 => {
                let value = self.reg('B') ^ self.combo(operand) as isize;
                self.set('B', value);
                ptr += 2;
            }
            2 => {
                self.set('B', self.combo(operand) % 8);
                ptr += 2;
            }
            3 => {
                if self.reg('A') != 0 {
                    ptr = operand as usize;
                } else {
                    ptr += 2;
                }
            },
            4 => {
                self.set('B', self.reg('B') ^ self.reg('C'));
                ptr += 2;
            }
            5 => {
                self.out.push(self.combo(operand) % 8);
                ptr += 2;
            }
            6 => {
                let value = self.reg('A') / 2_isize.pow(self.combo(operand) as u32);
                self.set('B', value);
                ptr += 2;
            }
            7 => {
                let value = self.reg('A') / 2_isize.pow(self.combo(operand) as u32);
                self.set('C', value);
                ptr += 2;
            }
            _ => panic!("Invalid opcode.")
        }
        if ptr < self.program.len() {
            self.process(ptr);
        }
    }

    fn out(&self) -> String {
        self.out.iter().map(|x| x.to_string()).join(",")
    }
}


#[aocd(2024, 17)]
pub fn solution1() {
    let data = input!();
    let mut machine = Machine::from(data.as_str());
    machine.process(0);
    submit!(1, machine.out());
}

#[aocd(2024, 17)]
pub fn solution2() {
    
}