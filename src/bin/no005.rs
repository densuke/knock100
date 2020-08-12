use std::io::{self, Write};

fn main() {
    print!("input 1st number: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let num1: i32 = input.parse().unwrap();
    print!("input 2nd number: ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let num2: i32 = input.parse().unwrap();
    println!("和: {}", num1+num2);
    println!("差: {}", num1-num2);
    println!("積: {}", num1*num2);
    println!("商: {}, 余り: {}", num1/num2 , num1 % num2);

}