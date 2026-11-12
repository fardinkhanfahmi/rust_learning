pub fn run()
{
    let mut x:u8=1;
    println!{"Now value of x is {x}"};
    loop
    {
        x=x+x;
        println!{"Now value of x is {x}"};
        if x==0
        {
            break;
        }
    }
}