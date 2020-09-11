/**
 * No.053
 * 自然数の入力値を素因数分解して結果を表示するプログラムを作成せよ。
 * 【実行例、下線部は入力例】
 * $ ./knock53
 * input number: 840
 * 2 2 2 3 5 7
 */

 fn get_prime_factorization(number: u32) -> Vec<u32> {
     let mut number = number;
     let mut result = Vec::new();
     while number > 1 {
        for div in 2..=number {
            if number % div == 0 {
                result.push(div);
                number /= div;
                break;
            }
        }
     }
     result
 }



#[test]
fn test() {
    assert_eq!(vec![2, 2, 2, 3, 5, 7], get_prime_factorization(840));
    assert_eq!(vec![2, 2, 2], get_prime_factorization(8));

}


fn main() {
    let number = mylib::get_number_with_prompt("input number: ") as u32;
    let result = get_prime_factorization(number);
    let mut rv = Vec::new();
    for item in result {
        rv.push(item.to_string());
    }
    println!("{}", rv.join(" "));

}
