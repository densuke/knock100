use std::io::{self, Write};

fn get_number_with_prompt(prompt: &str) -> i32 {
    print!("{}", &prompt);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let num1: i32 = input.parse().unwrap();
    num1
}

fn main() {
    let num1: i32 = get_number_with_prompt("input 1st number: ");
    let num2: i32 = get_number_with_prompt("input 2nd number: ");
    println!("和: {}", num1+num2);
    println!("差: {}", num1-num2);
    println!("積: {}", num1*num2);
    println!("商: {}, 余り: {}", num1/num2 , num1 % num2);

}