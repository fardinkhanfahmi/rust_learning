use std::io;
struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
pub fn run()
{
    let mut user_1=User{
    username:String::from("fardin"),
    email:String::from("fardin_khan_fahmi"),
    sign_in_count:80,
    active:true,
    };
     let s1 = user_1.username;
    user_1.username=String::from("fardin_khan");
     println!("Your username is {} and s1 is {s1}",user_1.username);
}