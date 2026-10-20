use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
pub fn run()
{
    let a: u8 =match random_number().checked_add(56){ 
    Some(num)=>num,
    None =>
    {
       println!("Cannot add");
       return;
    }
};
}
fn random_number() ->u8
{
    200
}
