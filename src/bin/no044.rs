fn convert(yen: u32, doller: u32) -> (u32, u32) {
    let result_doller: u32 = yen / doller;
    let result_cent = (yen - (doller * result_doller)) * 100 / doller;
    (result_doller, result_cent)
}

#[test]
fn check() {
    let (doller, cent) = convert(10000, 120);
    assert_eq!(83, doller);
    assert_eq!(33, cent);
    let (doller, cent) = convert(15000, 125);
    assert_eq!(120, doller);
    assert_eq!(0, cent);

}

fn main() {
    let yen = mylib::get_number_with_prompt("何円?: ") as u32;
    let doller = mylib::get_number_with_prompt("1ドルは何円?: ") as u32;

    let (doller, cent) = convert(yen, doller);
    println!("{}円は{}ドル{}セント", yen, doller, cent);
}
