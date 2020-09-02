/**
 * No.050
 * 1から100までの値を繰り返しで表示するが、
 * 3の倍数の時はfoo、5の倍数の時はbarと数字の代わりに表示するプログラムを作成せよ。
 * なお、3と5の両方の倍数の時はfoobarと表示される。
 */


 fn foobar(number: i32) -> String {
     if number % 15 == 0 {
         "foobar".to_string()
     } else if number % 3 == 0 {
         "foo".to_string()
     } else if number % 5 == 0 {
         "bar".to_string()
     } else {
         number.to_string()
     }
 }


#[test]
fn test() {
    assert_eq!((4).to_string(), foobar(4));
    assert_eq!("foo".to_string(), foobar(3));
    assert_eq!("bar".to_string(), foobar(5));
    assert_eq!("foobar".to_string(), foobar(30));
}


fn main() {
    for i in 1..=100 {
        println!("{}", foobar(i));
    }
}
