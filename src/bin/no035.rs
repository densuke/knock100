fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let number = mylib::get_number_with_prompt("input number: ") as usize;
    println!("array[{}] = {}", number, array[number]);

}