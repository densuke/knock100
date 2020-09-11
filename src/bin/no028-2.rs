// メモリの限りでかい数に対応してみる
use num_bigint::BigUint;
fn main() {
    let number = mylib::get_number_with_prompt("input number: ");
    let mut result = BigUint::from(1u128);

    if number > 0 {
        for num in 1..=number {
            result = result * (num as u128);
            if num % 500 == 0 {
                println!("({})", num);
            }
        }
    }
    println!("factorical = {}", result);
}

