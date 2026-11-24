use std::io;
struct User{
    active: bool,
    sign_in_count: u64,
    email:String,
    username:String,
}
pub fn run()
{
    let user_1=User{
        email: String::from("fardinkhanfahmi@gmail.com"),
        active:true,
        username:String::from("fardin"),
        sign_in_count:0,
    };
    println!("Your username is {}",user_1.username);
}