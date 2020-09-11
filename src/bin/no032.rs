fn main() {
    for count in 1u8..=20 {
        print!("{}: ", count);
        match count % 5 {
            0 => println!("bar"),
            _ => println!("{}",count),
        }
    }
}

