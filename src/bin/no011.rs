// use std::io::{self, Write};


// 使わないと判明してる部分は警告が出るので、コメントにして隠しておこう
/*
fn get_number_with_prompt(prompt: &str) -> i32 {
    print!("{}", &prompt);
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    let value: i32 = input.parse().unwrap();
    value
}
*/

fn main() {
    // 意図的に(intentionally)使わない変数は_を付けると文法的にOKらしい
    for _num in 0..10 {
        println!("Hello World!");
    }
}