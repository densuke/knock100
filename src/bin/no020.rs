
fn main() {
    let number1 = mylib::get_number_with_prompt("input 1st value: ");
    let number2 = mylib::get_number_with_prompt("input 2nd value: ");
    let div = number1/number2;
    println!("{}", div);
    let mul = div * number2;
    println!("{}", mul);

}