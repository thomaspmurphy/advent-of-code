use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file.")
        .trim()
        .to_string();

    let mut floor = 0;

    for (position, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue,
        }

        if floor == -1 {
            println!("Santa enters the basement at position: {}", position + 1);
        }
    }

    println!("Santa ends up on floor: {}", floor);
}
