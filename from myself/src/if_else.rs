use std::io;
pub fn run()
{
    let mut a =String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a:i64= a.trim().parse().unwrap();
    if a==10 {println!("a={}.\n",a);}
}