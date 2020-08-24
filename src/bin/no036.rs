fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let number1 = mylib::get_number_with_prompt("input index1: ") as usize;
    let number2 = mylib::get_number_with_prompt("input index2: ") as usize;
    println!("{} * {} = {}",
        array[number1],
        array[number2],
        array[number1] * array[number2]);

}