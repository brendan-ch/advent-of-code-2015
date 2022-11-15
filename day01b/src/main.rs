fn main() {
    let input = include_str!("../input.txt");
    let mut floor = 0;
    let mut index = 1;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => println!("Unknown character encountered"),
        }

        if floor < 0 {
            println!("{index}");
            break;
        }

        index += 1;
    }

}