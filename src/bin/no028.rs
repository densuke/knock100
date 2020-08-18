fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    let mut result: u128 = 1;

    if number > 0 {
        for num in 1..=number {
            result *= num as u128;
        }
    }
    println!("factorical = {}", result);
}

