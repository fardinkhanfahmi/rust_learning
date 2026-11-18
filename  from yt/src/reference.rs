use std::io;
pub fn run()
{
    let mut s =String::from("S");
    let l= calculate_length(&s);
    println!("s is {s} and length of s is {l}");
}
fn calculate_length(s:&String)->usize
{
    s.len()
}