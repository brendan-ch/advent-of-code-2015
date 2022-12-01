use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input.txt");

    let mut num = 0;
    let mut first_value_complete = false;

    loop {
        num += 1;
        let result = md5::compute(format!("{}{}", input, num));
        let first_five = format!("{:x}", result);

        if first_five.starts_with("00000") && !first_value_complete {
            println!("{num}");
            first_value_complete = true;
        }
        
        if first_five.starts_with("000000") {
            println!("{num}");
            break;
        }
    }

    let elapsed = now.elapsed().as_micros();
    println!("Time elapsed: {elapsed} microseconds");
}


