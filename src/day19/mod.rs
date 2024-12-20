use aocd::*;
use cached::proc_macro::cached;

fn read_data(data: String) -> (Vec<String>, Vec<String>) {
    let input: Vec<&str> = data.split("\n\n").collect();
    let patterns: Vec<String> = input[0].split(", ").map(|s| s.to_string()).collect();
    let designs: Vec<String> = input[1].lines().map(|s| s.to_string()).collect();
    (patterns, designs)
}

fn get_tokens(input: &str, dictionary: &[String]) -> Vec<String> {
    dictionary
        .iter()
        .filter(|&token| input.contains(token))
        .cloned()
        .collect()
}

#[cached]
fn reduce(input: String, tokens: Vec<String>) -> usize {
    let mut sequence = 0;
    for token in tokens.iter() {
        if input.starts_with(token) {
            let remainder = input.clone().split_off(token.len());
            if remainder.is_empty() {
                sequence += 1;
            } else {
                sequence += reduce(remainder, tokens.clone());
            }
        }
    }
    sequence
}

#[aocd(2024, 19)]
pub fn solution1() {
    let data = input!();
    let (dictionary, designs) = read_data(data);
    let mut total = 0;
    for design in designs {
        total += (reduce(design.clone(), get_tokens(&design, &dictionary)) > 0) as usize;
    }
    submit!(1, total);
}

#[aocd(2024, 19)]
pub fn solution2() {
    let data = input!();
    let (dictionary, designs) = read_data(data);
    let mut total = 0;
    for design in designs {
        total += reduce(design.clone(), get_tokens(&design, &dictionary));
    }
    submit!(2, total);
}
