fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    if (number >= -10 && number < 0) || number >= 10 {
        println!("OK");
    } else {
        println!("NG");
    }
}