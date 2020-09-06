/**
 * No.048
 * 最初に2以上の整数値を入力し、次の規則で計算し、計算回数と計算結果を表示し、
 * 計算結果が1になるまで繰り返すプログラムを作成せよ。
 * 規則:ある値が偶数ならその値を1/2にする。奇数ならその値を3倍して1を足す。
 */


 fn change(number: u32) -> u32 {
     match number % 2 {
         0 => number/2,
         _ => number*3+1
     }
 }

#[test]
fn changetest() {
    assert_eq!(6, change(12)); // 偶数は半分にする
    assert_eq!(10, change(3)); // 奇数は3倍にして+1(3*3+1=10)
}

fn main() {
    let mut number = mylib::get_number_with_prompt("input number: ") as u32;

    while number != 1 {
        print!("{}: ", number);
        number = change(number);
        println!("{}", number);
    }
}
