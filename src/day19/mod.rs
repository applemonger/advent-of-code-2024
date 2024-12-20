use aocd::*;
use cached::proc_macro::cached;

fn read_data(data: &str) -> (Vec<&str>, Vec<&str>) {
    let input: Vec<&str> = data.split("\n\n").collect();
    let patterns: Vec<&str> = input[0].split(", ").collect();
    let designs: Vec<&str> = input[1].lines().collect();
    (patterns, designs)
}

fn get_tokens(input: &str, dictionary: &[&str]) -> Vec<String> {
    dictionary
        .iter()
        .filter(|&token| input.contains(token))
        .map(|s| s.to_string())
        .collect()
}

#[cached]
fn combos(input: String, tokens: Vec<String>) -> usize {
    let mut sequence = 0;
    for token in tokens.iter() {
        if input.starts_with(token) {
            let remainder = input[token.len()..].to_string();
            if remainder.is_empty() {
                sequence += 1;
            } else {
                sequence += combos(remainder, tokens.clone());
            }
        }
    }
    sequence
}

#[aocd(2024, 19)]
pub fn solution1() {
    let data = input!();
    let (dictionary, designs) = read_data(data.as_str());
    let mut total = 0;
    for design in designs {
        total += (combos(design.to_string(), get_tokens(design, &dictionary)) > 0) as usize;
    }
    submit!(1, total);
}

#[aocd(2024, 19)]
pub fn solution2() {
    let data = input!();
    let (dictionary, designs) = read_data(data.as_str());
    let mut total = 0;
    for design in designs {
        total += combos(design.to_string(), get_tokens(design, &dictionary));
    }
    submit!(2, total);
}
