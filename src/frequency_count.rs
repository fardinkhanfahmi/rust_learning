use std::collections::HashMap;
use std::io;
pub fn run() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();
    let mut freq = HashMap::new();
    for c in input.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    for (ch, count) in freq {
        println!("{} -> {}", ch, count);
    }
}
