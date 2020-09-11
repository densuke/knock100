use std::io::{self, Write};

fn main() {
    print!("input number: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let result: i32 = input.parse().unwrap();
    let result = result * 3;
    println!("answer = {}", result);
}