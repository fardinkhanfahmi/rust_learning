use std::io;
pub fn run()
{
    let s ="Hello, Fardin";
    {
        let x = s;
        println!("x is {x}");
    }
    //println!("x is {x}");
    println!("s is {s}");
}