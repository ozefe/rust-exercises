use std::io::{self, Write};

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
fn main() {
    print!("Please write a sentence to be piglatinized: ");
    io::stdout()
        .flush()
        .expect("Failed writing to standard output");

    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed reading standard input");

    if sentence.len() <= 1 {
        panic!("Sentence length cannot be smaller than 2.");
    }

    let mut piglatinized: Vec<String> = Vec::new();

    for word in sentence.split_whitespace() {
        piglatinized.push(piglatinize(word));
    }

    println!("Piglatinized: {}", piglatinized.join(" "));
}

fn piglatinize(word: &str) -> String {
    if word.len() <= 1 {
        return word.to_string();
    }

    let word = word.trim().to_lowercase();

    let mut word_iter = word.chars();
    let first_char: char = word_iter.next().unwrap();
    let word_rest: String = word_iter.collect();

    if VOWELS.contains(&first_char) {
        return format!("{word}-hay");
    }

    format!("{word_rest}-{first_char}ay")
}
