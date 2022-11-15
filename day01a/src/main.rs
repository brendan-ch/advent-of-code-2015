use std::time::Instant;

fn main() {
    let now = Instant::now();
    
    let input = include_str!("../input.txt");
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("Unknown character encountered"),
        }
    }

    println!("{floor}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}