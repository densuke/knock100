fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let mut index = 0;
    for _ in 0..10 {
        println!("{}", array[index] - array[index+1]);
        index += 1;
    }
}