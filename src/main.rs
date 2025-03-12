use std::io;

fn main() {
    println!("Simple Rust Calculator | CLI");
    let mut first_num = String::new();
    io::stdin()
    .read_line(&mut first_num)
    .expect("Not a valid input!");

    let first_num: f32 = first_num.trim().parse().expect("Input is not a valid number!");

    
}
