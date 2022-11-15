use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input.txt");
    let mut sum = 0;

    for line in input.lines() {
        // vectors slow, arrays fast
        let mut d = [0, 0, 0];
        let mut next = 0;
        for i in line.split('x') {
            d[next] = i.parse::<i32>().expect("Unknown input");
            next += 1;
        }

        let min = (d[0] * d[1]).min(d[0] * d[2]).min(d[1] * d[2]);
        sum += min + (2 * d[0] * d[1]) + (2 * d[0] * d[2]) + (2 * d[1] * d[2]);
    }

    println!("{sum}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}
