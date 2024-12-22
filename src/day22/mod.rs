use std::collections::HashMap;

use aocd::*;

type Seq = (isize, isize, isize, isize);

fn gen_last(mut num: usize, n: usize) -> usize {
    for _ in 0..n {
        num = gen(num);
    }
    num
}

fn gen(mut num: usize) -> usize {
    num ^= num * 64;
    num = num % 16777216;
    num ^= num / 32;
    num = num % 16777216;
    num ^= num * 2048;
    num = num % 16777216;
    num
}

fn seq_map(mut num: usize, n: usize) -> HashMap<Seq, usize> {
    let mut prices = vec![num as isize % 10];
    for _ in 0..n {
        num = gen(num);
        let price = num % 10;
        prices.push(price as isize);
    }
    let diff: Vec<isize> = prices.windows(2).map(|x| x[1] - x[0]).collect();
    let mut changes = HashMap::<Seq, usize>::new();
    diff.windows(4).enumerate().for_each(|(i, diffs)| {
        let seq = (diffs[0], diffs[1], diffs[2], diffs[3]);
        if !changes.contains_key(&seq) {
            changes.insert(seq, prices[i+4] as usize);
        }
    });
    changes
}

fn merge(mut a: HashMap<Seq, usize>, b: HashMap<Seq, usize>) -> HashMap<Seq, usize> {
    for (k, v) in b {
        *a.entry(k).or_insert(0) += v;
    }
    a
}

#[aocd(2024, 22)]
pub fn solution1() {
    let data = input!();
    let secrets: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    let total: usize = secrets.iter().map(|&x| gen_last(x, 2000)).sum();
    submit!(1, total);
}

#[aocd(2024, 22)]
pub fn solution2() {
    let data = input!();
    let secrets: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    let mut totals = HashMap::<Seq, usize>::new();
    for secret in secrets {
        let map = seq_map(secret, 2000);
        totals = merge(totals, map);
    }
    let (_, best) = totals.iter().max_by_key(|&(_, &v)| v).unwrap();
    submit!(2, *best);
}