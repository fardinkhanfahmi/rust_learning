use std::io;
pub fn run()
{
    let mut s1=String::from("Hello");
    let l =add_str_calculate_length(&mut s1);
    println!("new s1 is {s1} and length of s1 is {l}.");
}
fn add_str_calculate_length(s1:&mut String)->usize
{
    s1.push_str(" World.");
    s1.len()
}