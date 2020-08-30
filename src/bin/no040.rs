fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    let result = match number % 2 {
        0 => "even",
        _ => "odd"
    };
    println!("{} is {}", number, result);
}