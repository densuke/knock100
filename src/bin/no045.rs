fn taxi(metor: u32) -> u32 {
    const FIRST_LEN: u32 = 1700; // 初乗りの距離
    const FIRST: u32 = 610; // 初乗り運賃

    const LEN: u32  = 313; // 値上げ距離
    const MORE: u32 = 80; // 距離別追加料金

    if metor <= FIRST_LEN {
        FIRST
    } else {
        let rest = metor - FIRST_LEN;
        let unit = rest / LEN + if rest % LEN != 0 { 1 } else { 0 };
        FIRST + unit * MORE
    }
}

#[test]
fn check() {
    let price = taxi(1500);
    assert_eq!(610, price);
    let price = taxi(10000);
    assert_eq!(2770, price);
    let price = taxi(2013);
    assert_eq!(690, price);
    let price = taxi(2014);
    assert_eq!(770, price);
}

fn main() {
    let metor = mylib::get_number_with_prompt("距離 ") as u32;
    println!("料金 {}", taxi(metor));
}
