fn main() {
    let n = mylib::get_number_with_prompt("input number: ");
    if n > 0 {
        for count in 0..n {
            print!("*");
            if (count+1) % 5 == 0 {
                print!(" ")
            }
        }
        println!("");
    }
}

