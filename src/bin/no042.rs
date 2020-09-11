fn main() {
    let number1 = mylib::get_number_with_prompt("input number1: ");
    let number2 = mylib::get_number_with_prompt("input number2: ");
    let number3 = mylib::get_number_with_prompt("input number3: ");
    if number1 <= number2 && number2 <= number3 {
        println!("OK");
    } else {
        println!("NG");
    }
}
