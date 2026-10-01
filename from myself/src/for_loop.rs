use std::io;
pub fn run()
{
    let mut a=String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut b:i64=a.trim().parse().unwrap();
    for i in 0..b
    {
        print!("{}",i);
    }
    for i in 0..=b
    {
        println!("{}",i);
    }
    for i in (0..b).rev()
    {
        println!("{}",i);
    }
    for i in (0..=b).rev()
    {
        println!("{}",i);
    }
}