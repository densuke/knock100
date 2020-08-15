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
    loop {
        let num = get_number_with_prompt("input number: ");
        if num == 0 {
            break;
        }
    }
}