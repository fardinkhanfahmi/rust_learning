use std::io;
pub fn run()
{
    let mut a1= String::new();
    io::stdin().read_line(&mut a1).unwrap();
    let mut a:f64 = a1.trim().parse().unwrap();
    let mut a1= String::new();
    io::stdin().read_line(&mut a1).unwrap();
    let mut b:f64 = a1.trim().parse().unwrap();
    a+=b;
    println!("{}\n",a);
}