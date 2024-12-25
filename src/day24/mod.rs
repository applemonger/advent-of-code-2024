use std::collections::HashMap;

use aocd::*;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Wire<'a> {
    Gates(&'a str, &'a str, &'a str),
    Bit(u8),
}

impl<'a> Wire<'a> {
    fn value(&self, lookup: &HashMap<&str, Wire>) -> u8 {
        match self {
            Wire::Gates(a, b, op) => match *op {
                "AND" => lookup[a].value(lookup) & lookup[b].value(lookup),
                "OR" => lookup[a].value(lookup) | lookup[b].value(lookup),
                "XOR" => lookup[a].value(lookup) ^ lookup[b].value(lookup),
                _ => panic!("Invalid operation!"),
            },
            Wire::Bit(bit) => *bit,
        }
    }
}

fn read_wires(input: &str) -> HashMap<&str, Wire> {
    input
        .lines()
        .flat_map(|line| {
            let bit_wire_re = Regex::new(r"(.*): (\d)").unwrap();
            let gate_wire_re = Regex::new(r"(.+) (.+) (.+) -> (.+)").unwrap();
            if let Some(cap) = bit_wire_re.captures(line) {
                let name = cap.get(1).unwrap().as_str();
                let bit = cap[2].parse::<u8>().unwrap();
                let wire = Wire::Bit(bit);
                Some((name, wire))
            } else if let Some(cap) = gate_wire_re.captures(line) {
                let a = cap.get(1).unwrap().as_str();
                let op = cap.get(2).unwrap().as_str();
                let b = cap.get(3).unwrap().as_str();
                let name = cap.get(4).unwrap().as_str();
                let wire = Wire::Gates(a, b, op);
                Some((name, wire))
            } else {
                None
            }
        })
        .collect()
}

#[aocd(2024, 24)]
pub fn solution1() {
    let data = input!();
    let wires = read_wires(&data);
    let mut out = Vec::new();
    for z_wire in wires.keys().filter(|k| k.starts_with('z')).sorted() {
        let bit = wires.get(z_wire).unwrap().value(&wires);
        out.insert(0, bit);
    }
    let n = out.iter().fold(0, |n, &bit| (n << 1) | (bit as u64));
    submit!(1, n);
}

#[aocd(2024, 24)]
pub fn solution2() {}
