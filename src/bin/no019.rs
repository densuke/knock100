extern crate mylib;

fn main() {
    let mut a = [0; 5]; // とりあえず5つの整数用配列

    for i in 0..a.len() {
        a[i] = mylib::get_number_with_prompt("input number: ");
    }
    for i in &a {
        println!("{}", i);
    }
}