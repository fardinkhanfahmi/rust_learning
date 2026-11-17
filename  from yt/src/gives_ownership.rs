use std::io;
pub fn run()
{
    let mut m = String::from("M");
    m=gives_ownership(m);
    println!("m is {m}");
}
fn gives_ownership(n:String)->String
{
    println!("n is {n}");
    let mut n = String::from("new m");
    n
}