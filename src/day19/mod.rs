use aocd::*;
use cached::proc_macro::cached;

fn read_data(data: String) -> (Vec<String>, Vec<String>) {
    let input: Vec<&str> = data.split("\n\n").collect();
    let patterns: Vec<String> = input[0].split(", ").map(|s| s.to_string()).collect();
    let designs: Vec<String> = input[1].lines().map(|s| s.to_string()).collect();
    (patterns, designs)
}

fn fragment(input: String, token: String, start: usize) -> Vec<String> {
    let mut fragments = Vec::new();
    let end = start + token.len();
    let first = input[..start].to_string();
    if !first.is_empty() {
        fragments.push(first);
    }
    let second = input[end..].to_string();
    if !second.is_empty() {
        fragments.push(second);
    }
    fragments
}

fn get_tokens(input: &String, dictionary: &Vec<String>) -> Vec<String> {
    dictionary.iter().filter(|&token| input.contains(token)).cloned().collect()
}

fn indices(s: String, token: String) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut start = 0;
    while let Some(pos) = s[start..].find(&token) {
        let index = start + pos;
        indices.push(index);
        start = index + 1;
    }
    indices
}

// fn reduce(input: String, dictionary: &HashSet<String>, mut graph: HashMap<String, HashSet<String>>) -> HashMap<String, HashSet<String>> {
//     for token in get_tokens(&input, dictionary) {
//         graph.entry(input.clone()).or_default().insert(token.clone());
//         for i in indices(input.clone(), token.clone()) {
//             let fragments = fragment(input.clone(), token.clone(), i);
//             for fragment in fragments {
//                 graph = reduce(fragment.clone(), dictionary, graph);
//             }
//         }
//     }
//     graph
// }

#[cached]
fn reducable(input: String, dictionary: Vec<String>) -> bool {
    let tokens = get_tokens(&input, &dictionary);
    if tokens.is_empty() {
        return false;
    }
    for token in get_tokens(&input, &dictionary) {
        for i in indices(input.clone(), token.clone()) {
            let mut reduced = true;
            for fragment in fragment(input.clone(), token.clone(), i) {
                reduced &= reducable(fragment.clone(), dictionary.clone());
            }
            if reduced {
                return true;
            }
        }
    }
    false
}

// fn recreate(input: String, graph: &HashMap<String, HashSet<String>>) -> bool {
//     if let Some(tokens) = graph.get(&input) {
//         for token in tokens.iter() {
//             for i in indices(input.clone(), token.clone()) {
//                 let fragments = fragment(input.clone(), token.clone(), i);
//                 let mut tractable = true;
//                 for fragment in fragments {
//                     tractable &= recreate(fragment, graph);
//                 }
//                 if tractable {
//                     return true;
//                 }
//             }
//         }
//         false
//     } else {
//         false
//     }
// }

#[aocd(2024, 19)]
pub fn solution1() {
    let data = input!();
    let (dictionary, designs) = read_data(data);
    let mut possible = 0;
    for design in designs {
        let reduced = reducable(design.clone(), dictionary.clone());
        println!("{} {:?}", reduced, design);
        possible += reduced as usize;
    }
    submit!(1, possible);
}

#[aocd(2024, 19)]
pub fn solution2() {
    
}