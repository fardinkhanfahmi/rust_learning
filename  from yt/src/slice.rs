use std::io;
pub fn run()
{
    let s=String::from("Hello World");
    let res=find_space(&s);
    println!("result is {res}");
}
fn find_space(s:&String)->&str
{
   let f=s.as_bytes();
   for(i,&value) in f.iter().enumerate()
   {
     if value==b' '
     {
        return &s[..i];
     }
   }
   return &s[..]
}