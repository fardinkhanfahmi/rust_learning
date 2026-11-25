use std::io;
struct user{
    username:String,
    email:String,
    active:bool,
    sign_in_count:u64,
}
pub fn run()
{
    let mut user1=build_user(
        String::from("fardin"),
        String::from("fardinkhanfahmi@gmail.com"),
    );
    println!("Your username is {}",user1.username);
}
fn build_user(username:String,email:String)->user
{
    user{
    username,
    email,
    active:true,
    sign_in_count:0,
    }
}