extern crate mylib;
// Vec版

fn main() {
    let length = 5;
    let mut a: Vec<i32> = Vec::new();

    for _ in 0..length {
        a.push(mylib::get_number_with_prompt("input number: "));
    }

    for i in a {
        println!("{}", i);
    }
}
