
use std::io::{self, Read};

pub fn run() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a = [0; 5];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
    }
    println!("{:?}", a);
}