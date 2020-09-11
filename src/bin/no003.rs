use std::io::{Write, self};

fn main() {
    let mut input = String::new();
    print!("input number:");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).ok();
    println!("your number is: {}", input);

}