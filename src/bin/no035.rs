fn main() {
    let array = vec![3u8, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let number = mylib::get_number_with_prompt("input number: ");
    println!("array[{}] = {}", number, array.get(number as usize).unwrap());

}