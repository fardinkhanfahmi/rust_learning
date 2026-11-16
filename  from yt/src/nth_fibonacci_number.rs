use std::io;
pub fn run()
{
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n:i32= n.trim().parse().unwrap();
    println!("N-th fibonacci number is {}",fibo(n));
}
fn fibo(n:i32)->i32
{
    if n<=1
    {return n;}
    return fibo(n-1)+fibo(n-2);
}