fn main() {
    let mut result = 0;
    for _ in 0..5 {
        result += mylib::get_number_with_prompt("input number: ");
    }
    println!("sum = {}", result);
}

