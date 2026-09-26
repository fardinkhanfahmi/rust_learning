use std::io;
pub fn run()
{
    let mut a =String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a : f64 = a.trim().parse().unwrap();
    println!("{}\n",a);
}