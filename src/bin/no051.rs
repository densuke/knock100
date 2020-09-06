/**
 * No.051
 * 指定した金額を100円玉と10円玉と1円玉だけで、できるだけ少ない枚数で支払いたい。
 * 金額を入力するとそれぞれの枚数を計算して表示するプログラムを作成せよ。
 * 例: $ ./knock51
 * input money: 12345
 * 100円玉123枚, 10円玉4枚, 1円玉5枚
 */


 fn check_coin(price: u32) -> (u32, u32, u32) {
     let coins_100 = price / 100;
     let rest = price - coins_100 * 100;
     let coins_10 = rest / 10;
     let rest = rest - coins_10 * 10;
     (coins_100, coins_10, rest)
 }


#[test]
fn test() {
    assert_eq!((0,0,0), check_coin(0));
    assert_eq!((1,1,0), check_coin(110));
    assert_eq!((1,1,1), check_coin(111));
    assert_eq!((123,4,5), check_coin(12345));

}


fn main() {
    let price = mylib::get_number_with_prompt("input money: ") as u32;
    let(num_100, num_10, num_1) = check_coin(price);
    println!("100円玉{}枚, 10円玉{}枚, 1円玉{}枚", num_100, num_10, num_1);
}
