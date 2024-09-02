use std::io::{self, Write};

fn main() {
    println!("Temperature Converter");
    println!("Supported degrees: Fahrenheit (F), Celsius (C)");

    print!(
        "Please type in the temperature you want to convert, ending with either F or C (e.g. 23C): "
    );
    io::stdout()
        .flush()
        .expect("Failed writing to standard output");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let mut user_input = user_input.trim().to_string();

    let degree = user_input.pop();
    let temperature: i32 = user_input.parse().expect("Temperature must be a number");

    match degree {
        Some('F') => println!("{temperature}F is {}C", (temperature - 32) * 5 / 9),
        Some('C') => println!("{temperature}C is {}F", (temperature * 9 / 5) + 32),
        _ => panic!("Degree must be either Fahrenheit (F) or Celsius (C)!"),
    }
}
