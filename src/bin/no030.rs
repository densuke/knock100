fn main() {
    let n = mylib::get_number_with_prompt("input number: ");
    if n > 0 {
        for _ in 0..n {
            print!("*");
        }
        println!("");
    }
}

