extern crate mylib;
use mylib::get_number_with_prompt;

fn main() {
    let print_count = get_number_with_prompt("input number: ");
    for num in 0..=print_count {
        println!("{}", print_count - num);
    }
}