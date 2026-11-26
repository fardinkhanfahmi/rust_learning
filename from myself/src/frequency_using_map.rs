use std::collections::HashMap;

pub fn run() {
    let s = String::from("hello world");

    let mut freq = HashMap::new();

    for ch in s.chars() {
        let count = freq.entry(ch).or_insert(0);
        *count += 1;
    }

    for (ch, count) in &freq {
        println!("{ch} -> {count}");
    }
}