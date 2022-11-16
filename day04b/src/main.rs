use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input.txt");

    let mut num = 0;
    let mut first_five = "".to_string();

    while !first_five.starts_with("000000") {
        num += 1;
        let result = md5::compute(input.to_string() + &num.to_string());
        first_five = format!("{:x}", result);
    }
    println!("{num}");

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}


