fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    let range = if number < -10 { 1 }
                else if number >= 0 { 3 }
                else {2};
    println!("range {}", range);
}