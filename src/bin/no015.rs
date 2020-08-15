use std::io::{self, Write};


fn get_number_with_prompt(prompt: &str) -> i32 {
    print!("{}", &prompt);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let value: i32 = input.parse().unwrap();
    value
}

fn main() {
    let print_count = get_number_with_prompt("input number: ");
    for num in (0..=print_count).filter(|n| n % 2 == 0) {
        println!("{}", num);
    }
}