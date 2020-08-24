fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let number = mylib::get_number_with_prompt("input index: ") as usize;
    println!("value = {}", array[array[number]]);

}