use std::io::{self, Write};

fn main() {
    let mut first_num = String::new();
    let mut second_num = String::new();
    let mut operation = String::new();

    println!("Simple Rust Calculator | CLI");

    print!("Please input the calculation operation! [+|-|*|/]: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin()
        .read_line(&mut operation)
        .expect("The input you provided is invalid!");

    print!("Please input the first number: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin()
    .read_line(&mut first_num)
    .expect("The input you provided is invalid!");
    let first_num: f32 = first_num.trim().parse().expect("Input is not a valid number!");

    print!("Please input the second number: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin()
    .read_line(&mut second_num)
    .expect("The input you provided is invalid!");
    let second_num: f32 = second_num.trim().parse().expect("Input is not a valid number!");

    let result = match operation.trim() {
        "+" => first_num + second_num,
        "-" => first_num - second_num,
        "*" => first_num * second_num,
        "/" => first_num / second_num,
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    println!("Result: {result}");
}
