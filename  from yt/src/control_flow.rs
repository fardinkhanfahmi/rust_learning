pub fn run()
{
    let y = is_even(4);
    println!("Value of y is {y}");
    let x=if y{10}else{20};
    println!("Value of x is {x}");
}
fn is_even(x:i32)->bool
{
    if x%2==0
    {
        true
    }
    else
    {
        false
    }
}