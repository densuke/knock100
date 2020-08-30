/**
 * No.047
 * 異なる整数値を2つ入力し、それぞれ別の変数に格納する。
 * そして、それらの変数の値を入れ替えた後、
 * それぞれの変数の値を表示するプログラムを作成せよ。
 * 単に順序を変えて表示するだけではダメ。必ず変数の値を入れ替えること。
 * 下の実行例の場合、まず変数aに2、bに5が入力された状態から、
 * aの値が5、bの値が2になるように入れ替える。
 */

fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}


fn main() {
    let number_a = mylib::get_number_with_prompt("input a: ");
    let number_b = mylib::get_number_with_prompt("input b: ");
    let (number_a, number_b) = swap(number_a, number_b);
    println!("a = {}, b = {}", number_a, number_b);
}
