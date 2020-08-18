fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "others"
    };
    println!("{}", result);
}

