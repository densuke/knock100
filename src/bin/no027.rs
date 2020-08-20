fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    let mut result = 0;
    if number > 0 {
        for num in 1..=number {
            result += num;
        }
    }
    println!("sum = {}", result);
}

