use std::io;
pub fn run()
{
    let mut a= String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a:i32 = a.trim().parse().unwrap();
    if a==0 || a==100 {println!("a=0 or a=100");}
    else if a>0 && a<100 {println!("a is between 0 to 100");}
    if a>100 {println!("a is greater than 100");}
}