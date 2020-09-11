fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    if number > 5 && number < 20 {
        println!("OK");
    }
}