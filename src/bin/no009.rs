extern crate mylib;
use mylib::get_number_with_prompt;

fn main() {
    let num = get_number_with_prompt("input number: ");
    let str = if num > 0 { "positive" } else if num < 0 { "negative" } else {"zero"} ;
    println!("{}", str);
}