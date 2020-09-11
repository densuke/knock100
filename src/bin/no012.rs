extern crate mylib;
use mylib::get_number_with_prompt;

fn main() {
    let print_count = get_number_with_prompt("input number:");
    // 意図的に(intentionally)使わない変数は_を付けると文法的にOKらしい
    for _num in 0..print_count {
        println!("Hello World!");
    }
}