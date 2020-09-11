/**
 * No.054
 * まずデータの個数を入力させ、次にデータの個数だけ整数値を入力させる。
 * この入力データの中で最大値と最小値を求め表示するプログラムを作成せよ。
 * データの個数は100個までとする。
 * なお、データの個数とデータはファイルからリダイレクトで入力させればよいので、
 * 入力のためのメッセージは不要である(実行例を参照すること)。
 *
 * 【実行例、データファイルは下のリンクから取得せよ】
 * $ ./knock54 < small.data
 * 最小値 = 128, 最大値 = 962
 * $ ./knock54 < middle.data
 * 最小値 = 20, 最大値 = 988
 * $ ./knock54 < large.data
 * 最小値 = 5, 最大値 = 996
 */
use std::io;

fn find_max_and_min(list: Vec<u32>) -> (u32, u32) {
    let mut max = list[0];
    let mut min = list[0];
    for num in list {
        if max < num {
            max = num;
        } else if min > num {
            min = num;
        }
    }
    (max, min)
}

#[test]
fn test_find_max_and_min() {
    let sample = vec![5, 3, 4, 32];
    assert_eq!((32, 3), find_max_and_min(sample));
}

fn main() {
    let mut data = Vec::new();
    // input.push(5);
    // input.push(3);
    let input = io::stdin();
    for _ in 0..100 {
        let mut buf = String::new();
        match input.read_line(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                match buf.trim().parse() {
                    Ok(n) => {
                        data.push(n);
                    }
                    Err(_) => {
                        // nothing todo
                    }
                }
            }
            Err(_) => {}
        }
    }
    let (max, min) = find_max_and_min(data);
    println!("最小値 = {}, 最大値 = {}", max, min);
}
