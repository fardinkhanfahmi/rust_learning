use std::io;
pub fn run()
{
    let mut x= String::from("I am x");
    let mut y=x.clone();
    y.push_str("This is y");
    println!("x is {x} and y is {y}");
}