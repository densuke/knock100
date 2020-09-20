/**
 * No.055
 * 「とんで」を9回「まわって」を3回繰り返した後「まわる」と
 * 表示して改行する、を3回繰り返すプログラムを作成せよ。
 * 「とんで」「まわって」と3行文の繰り返しは必ず繰り返し構文を使うこと。
 *
 * 【実行例】
 * $ ./knock55
 * とんでとんでとんでとんでとんでとんでとんでとんでとんでまわってまわってまわってまわる
 * とんでとんでとんでとんでとんでとんでとんでとんでとんでまわってまわってまわってまわる
 * とんでとんでとんでとんでとんでとんでとんでとんでとんでまわってまわってまわってまわる
 */

fn tonde() -> String {
    let mut result = String::new();
    for _ in 0..9 {
        result += "とんで"
    }
    for _ in 0..3 {
        result += "まわって"
    }

    result + "まわる"
}

#[test]
fn test_tonde() {
    let s = "とんでとんでとんでとんでとんでとんでとんでとんでとんでまわってまわってまわってまわる".to_string();
    assert_eq!(s, tonde());
}

fn main() {
    for _ in 0..3 {
        println!("{}", tonde());
    }
}