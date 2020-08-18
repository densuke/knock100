extern crate mylib;
use mylib::get_number_with_prompt;

fn main() {
    loop {
        let num = get_number_with_prompt("input number: ");
        if num == 0 {
            break;
        }
    }
}