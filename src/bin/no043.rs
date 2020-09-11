fn main() {
    // D = b^2 - 4ac
    // D > 0: 実数解2つ
    // D = 0: 重根
    // D < 0: 虚数解2つ
    let number_a = mylib::get_number_with_prompt("input a: ");
    let number_b= mylib::get_number_with_prompt("input b: ");
    let number_c= mylib::get_number_with_prompt("input c: ");
    let d = number_b * number_b - 4 * number_a * number_c;
    if d > 0 {
        println!("実数解");
    } else if d == 0 {
        println!("重解");
    } else {
        println!("虚数解");
    }
}
