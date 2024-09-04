use std::collections::HashSet;
use std::io;

fn remove_duplicates(input: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for &item in &input {
        if seen.insert(item) {
            result.push(item);
        }
    }

    result
}

fn main() {
    println!("aralarına bir boşluk bırakarak sayıları yazınız:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("hata");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let output = remove_duplicates(numbers);
    println!("Output: {:?}", output);
}
