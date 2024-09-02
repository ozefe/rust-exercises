use std::io::{self, Write};

fn main() {
    println!("Nth Fibonacci Number Generator");

    print!("Please type in the N: ");
    io::stdout()
        .flush()
        .expect("Failed writing to standard output");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed reading standard input");
    let n: u128 = n.trim().parse().expect("N must be a number!");

    println!("The {n}. Fibonacci number is {}", nth_fib(n));
}

fn nth_fib(n: u128) -> u128 {
    if n == 0 {
        0
    } else if (n == 1) | (n == 2) {
        1
    } else {
        nth_fib(n - 2) + nth_fib(n - 1)
    }
}
