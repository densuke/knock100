fn main() {
    let mut a = [0; 10];
    for i in 0..10 {
        a[i] = i;
    }

    for i in &a {
        println!("{}", i);
    }


}