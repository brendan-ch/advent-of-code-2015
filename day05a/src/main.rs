use std::time::Instant;
use fancy_regex::Regex;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input.txt");
    let regex_matcher1 = Regex::new(r"^(\w*?[aeiou]\w*?){3,}$").unwrap();
    let regex_matcher2 = Regex::new(r"\w*(.)\1{1}\w*").unwrap();
    let regex_matcher3 = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    let mut num_matches = 0;
    for line in input.split('\n') {
        if regex_matcher1.is_match(line).unwrap() && regex_matcher2.is_match(line).unwrap() && !regex_matcher3.is_match(line).unwrap() {
            num_matches += 1;
        }
    }

    println!("{num_matches}");
    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}


