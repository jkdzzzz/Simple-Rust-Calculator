use std::io::{self, Write};

fn main() {
    let mut first_num = String::new();
    let mut second_num = String::new();

    println!("Simple Rust Calculator | CLI");

    print!("Please input the first number: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin()
    .read_line(&mut first_num)
    .expect("Not a valid input!");
    let first_num: f32 = first_num.trim().parse().expect("Input is not a valid number!");

    print!("Please input the second number: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin()
    .read_line(&mut second_num)
    .expect("Not a valid input!");
    let second_num: f32 = second_num.trim().parse().expect("Input is not a valid number!");

    println!("{}", first_num + second_num);
}
