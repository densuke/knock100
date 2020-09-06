/**
 * No.052
 * 西暦を入力したらその年が閏(うるう)年かどうかを判定するプログラムを作成せよ。
 * なお、4で割り切れる年のうち、100で割り切れないか、400で割り切れる年は閏年である。
 * 例:
 * $ ./knock52
 * input year: 2015
 * 2015年は閏年でない
 *  * $ ./knock52
 * input year: 2016
 * 2016年は閏年である
 * $ ./knock52
 * input year: 2100
 * 2100年は閏年でない
 * $ ./knock52
 * input year: 2000
 * 2000年は閏年である
 */


 fn check_leapyear(year: u32) -> bool {
     if year % 4 != 0 || year % 400 != 0 && year % 100 == 0 {
         false
     } else {
         true
     }
 }


#[test]
fn test() {
    assert_eq!(false, check_leapyear(2015));
    assert_eq!(true, check_leapyear(2016));
    assert_eq!(false, check_leapyear(2100));
    assert_eq!(true, check_leapyear(2000));
}


fn main() {
    let year = mylib::get_number_with_prompt("input year: ") as u32;
    println!("{}年は閏年で{}",
        year,
        match check_leapyear(year) {
            true => "ある",
            false => "ない",
        }
    );
}
