extern crate mylib;

fn main() {
    let num1: i32 = mylib::get_number_with_prompt("input 1st number: ");
    let num2: i32 = mylib::get_number_with_prompt("input 2nd number: ");
    println!("和: {}", num1+num2);
    println!("差: {}", num1-num2);
    println!("積: {}", num1*num2);
    if num2 != 0 {
        println!("商: {}, 余り: {}", num1/num2 , num1 % num2);
    } else {
        println!("ゼロ除算を行おうとしています");
    }

}