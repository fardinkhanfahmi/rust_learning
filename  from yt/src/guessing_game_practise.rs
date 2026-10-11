use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
pub fn run()
{
    let a= rand::thread_rng().gen_range(1..100);
    loop
    {
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("Failed.\n");
        let b:i64 = match b.trim().parse()
        {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match b.cmp(&a)
        {
            Ordering::Less=>println!("{}",(b-a).to_string().red()),
            Ordering::Greater=>println!("{}",(b-a).to_string().red()),
            Ordering::Equal=>{println!("{}","OK".green());break;}
        }
    }
}