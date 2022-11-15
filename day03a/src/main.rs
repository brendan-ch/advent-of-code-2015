use std::{collections::HashSet, time::Instant};

fn main() {
    let now = Instant::now();

    // My solution: brute force w/ HashSet
    let input = include_str!("../input.txt");
    let mut coords = [0, 0];
    let mut prev_coords: HashSet<[i32; 2]> = HashSet::with_capacity(3000); // ?????

    input.chars().for_each(|c| {
        match c {
            '^' => coords[1] += 1,
            '>' => coords[0] += 1,
            'v' => coords[1] -= 1,
            '<' => coords[0] -= 1,
            _ => panic!("Unknown character encountered"),
        }
        
        prev_coords.insert(coords);
    });
    
    let prev_visited = prev_coords.len();
    println!("{prev_visited}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
