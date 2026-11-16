use std::io;
pub fn run()
{
    let m = String::from("M");
    take_ownership(m);
    println!("m is {m}");
}
fn take_ownership(n:String)
{
    println!("n is {n}");
}