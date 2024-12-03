use aocd::*;
use regex::Regex;

#[aocd(2024, 3)]
pub fn solution1() {
    let data = input!();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum: u64 = re.captures_iter(data.as_str())
        .map(|c| {
            let (_, [first, second]) = c.extract();
            let first = first.parse::<u64>().unwrap();
            let second = second.parse::<u64>().unwrap();
            first * second
        })
        .sum();
    submit!(1, sum);
}

#[aocd(2024, 3)]
pub fn solution2() {
    let data = input!();
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut score = 0;
    re.captures_iter(data.as_str())
        .fold(true, |mut enabled, capture| {
            let (_, [extract]) = capture.extract();
            if extract == "do()" {
                enabled = true;
            } else if extract == "don't()" {
                enabled = false;
            } else {
                let pair: Vec<u64> = extract.split(',').map(|num| num.parse::<u64>().unwrap()).collect();
                let first = *pair.first().unwrap();
                let second = *pair.last().unwrap();
                score += (enabled as u64) * first * second;
            }
            enabled
        });

    submit!(2, score);
}