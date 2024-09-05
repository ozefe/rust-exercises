use rand::Rng;
use std::{
    collections::HashMap,
    io::{self, Write},
    ops::RangeInclusive,
};

const MAX_VECTOR_SIZE: usize = 1024;
const DEFAULT_VECTOR_SIZE: usize = MAX_VECTOR_SIZE / 2;
const DEFAULT_RANGE: RangeInclusive<u8> = 0..=255;

fn main() {
    let mut integers: Vec<u8> = Vec::new();

    // Take user input for `vector_size`
    print!("Please provide a vector size for random integers: ");
    io::stdout()
        .flush()
        .expect("Failed writing to standard output");

    let mut vector_size = String::new();
    io::stdin()
        .read_line(&mut vector_size)
        .expect("Failed reading standard input");
    let vector_size: usize = vector_size.trim().parse().unwrap_or(DEFAULT_VECTOR_SIZE);

    // Check if `vector_size` is bigger than `MAX_VECTOR_SIZE`
    if vector_size > MAX_VECTOR_SIZE {
        panic!("Provided vector size ({vector_size}) cannot be larger than the defined maximum vector size ({MAX_VECTOR_SIZE})!");
    } else if vector_size == 0 {
        panic!("Provided vector size ({vector_size}) cannot be 0.");
    }

    // Generate random positive integers
    for _ in 0..vector_size {
        integers.push(rand::thread_rng().gen_range(DEFAULT_RANGE));
    }
    // dbg!(integers);

    // dbg!(get_median(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    println!("Median value: {}", get_median(&integers));

    // dbg!(get_mode(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10]));
    println!("Mode value: {}", get_mode(&integers));
}

fn get_median(v: &Vec<u8>) -> u8 {
    let mut v = v.clone();
    v.sort();
    v[v.len() / 2]
}

fn get_mode(v: &Vec<u8>) -> u8 {
    let mut counts: HashMap<u8, u8> = HashMap::new();
    for i in v {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }

    *counts.iter().max_by_key(|pair| pair.1).unwrap().0
}
