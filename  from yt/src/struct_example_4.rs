use std::io;
struct user{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
pub fn run()
{
    let mut user1=user{
        username:String::from("fardin"),
        email:String::from("fardinkhanfahmi@gmail.com"),
        active:true,
        sign_in_count:0,
    };
    let mut user2=user{
        username:String::from("fardin_2004"),
        ..user1
    };
    println!("You username is {}",user1.username);
}