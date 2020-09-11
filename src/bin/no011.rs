fn main() {
    // 意図的に(intentionally)使わない変数は_を付けると文法的にOKらしい
    for _num in 0..10 {
        println!("Hello World!");
    }
}