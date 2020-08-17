extern crate mylib;

fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    let a = [number; 10]; // 実行時にスタック上で確保してるということか?

    for i in &a {
        println!("{}", i);
    }
}