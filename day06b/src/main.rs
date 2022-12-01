use std::time::Instant;
use fancy_regex::Regex;

fn main() {
    let now = Instant::now();
    let input = include_str!("../input.txt");

    let mut grid: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    // Construct regex matcher
    let regex = Regex::new(r"\d+").unwrap();

    for line in input.split('\n') {
        let coords1: (i16, i16);
        let coords2: (i16, i16);
        // Match coordinates in string
        let mut regex_result = regex.find_iter(line);

        coords1 = (
            regex_result.next().unwrap().unwrap().as_str().parse::<i16>().unwrap(),
            regex_result.next().unwrap().unwrap().as_str().parse::<i16>().unwrap(),
        );
        
        coords2 = (
            regex_result.next().unwrap().unwrap().as_str().parse::<i16>().unwrap(),
            regex_result.next().unwrap().unwrap().as_str().parse::<i16>().unwrap(),
        );

        // compare with grid
        for x in coords1.0..=coords2.0 {
            for y in coords1.1..=coords2.1 {
                if line.starts_with("turn on") {
                    grid[x as usize][y as usize] += 1;
                } else if line.starts_with("toggle") {
                    grid[x as usize][y as usize] += 2;
                } else if grid[x as usize][y as usize] - 1 >= 0 {
                    grid[x as usize][y as usize] -= 1;
                }
            }
        }
    }

    let mut brightness: i32 = 0;
    for row in grid {
        for item in row {
            brightness += item;
        }
    }
    println!("{brightness}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
