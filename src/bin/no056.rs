/**
 * No.055
 * 0〜65535の整数値を入力させ、入力値を16桁の2進数に変換して表示するプログラムを作成せよ。
 * 【実行例】
 * $ ./knock56
 * input number: 127
 * 0000000001111111
 * $ ./knock56
 * input number: 10000
 * 0010011100010000
 * $ ./knock56
 * input number: 65535
 * 1111111111111111
 */

fn to_bin(number: u16) -> String {
    let mut number = number;
    let mut result = String::new();
    for _ in 0..16 {
        result += &(number % 2).to_string();
        number /= 2;
    }

    result.chars().rev().collect::<String>()

//    result
}

#[test]
fn test_bin() {
    assert_eq!(to_bin(3), "0000000000000011");
    assert_eq!(to_bin(65535), "1111111111111111");
}

fn main() {
    let num = mylib::get_number_with_prompt("input number: ") as u16;
    println!("{}", to_bin(num));
}
