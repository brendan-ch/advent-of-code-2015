use std::time::Instant;
use std::u32::MAX;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input.txt");
    let mut sum = 0;

    for line in input.lines() {
        // vectors slow, arrays fast
        let mut d: [u32; 3] = [0, 0, 0];
        let mut next = 0;
        let mut min2 = MAX;
        let mut min1 = MAX;

        for i in line.split('x') {
            d[next] = i.parse::<u32>().expect("Unknown input");
            if d[next] < min1 {
                min2 = min1;
                min1 = d[next];
            } else if d[next] < min2 {
                min2 = d[next];
            }

            next += 1;
        }

        sum += (d[0] * d[1] * d[2]) + (min1 * 2) + (min2 * 2);
    }

    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
