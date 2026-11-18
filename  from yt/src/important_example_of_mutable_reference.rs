use std::io;
pub fn run()
{
    let mut s=String::from("Hello");
    let s1 = &s;
    let s2=&s;
    println!("{s1} {s2}");
    let s3= &mut s;
    println!("{s3}");
}