use std::io;
pub fn run()
{
    let mut s = String::from("hello");
    s.push_str(" World");
    println!("s = {s}");
}