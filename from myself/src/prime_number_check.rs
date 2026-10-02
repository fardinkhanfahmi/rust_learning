use std::io;
pub fn run()
{
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a:i32= a.trim().parse().unwrap();
    for i in 2..a
    {
        if i*i>a{println!("{} is a prime number.",a);return;}
        else if a%i==0 {println!("{} is not a prime number.",a);return;}
    }
}