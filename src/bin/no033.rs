fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    for count in 1..=9 {
        if count != number {
            println!("{}", count);
        }
    }
}