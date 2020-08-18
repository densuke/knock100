extern crate mylib;
use mylib::get_number_with_prompt;

fn main() {
    let num = get_number_with_prompt("input number: ");
    let num = if num < 0 { -1 * num } else { num };
    println!("absolute value is {}", num);
}