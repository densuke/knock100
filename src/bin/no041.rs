fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    println!(
        "{} is {}a single figure.",
        number,
        if number > 0 && number < 10 {
            ""
        } else {
            "not "
        }
    );
}
