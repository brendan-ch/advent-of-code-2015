use std::{collections::HashSet, time::Instant};

fn main() {
    let now = Instant::now();

    // My solution: brute force w/ HashSet
    let input = include_str!("../input.txt");
    let mut coords1 = [0, 0];
    let mut coords2 = [0, 0];
    let mut prev_coords: HashSet<[i32; 2]> = HashSet::with_capacity(3000); // ?????

    let mut robot = false;

    input.chars().for_each(|c| {
        let mut temp_coords;
        if robot {
            temp_coords = coords1;
        } else {
            temp_coords = coords2;
        }
        
        match c {
            '^' => temp_coords[1] += 1,
            '>' => temp_coords[0] += 1,
            'v' => temp_coords[1] -= 1,
            '<' => temp_coords[0] -= 1,
            _ => panic!("Unknown character encountered"),
        }

        if robot {
            coords1 = temp_coords;
        } else {
            coords2 = temp_coords;
        }
        
        prev_coords.insert(temp_coords);
        robot = !robot;
    });
    
    let prev_visited = prev_coords.len();
    println!("{prev_visited}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
