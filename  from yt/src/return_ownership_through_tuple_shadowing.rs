use std::io;
pub fn run()
{
    let mut m = String::from("M");
    let(m,l)=gives_ownership(m);
    println!("m is {m}, l is {l}");
}
fn gives_ownership(n:String)->(String,usize)
{
    let l=n.len();
    (n,l)
}