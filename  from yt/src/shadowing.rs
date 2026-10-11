pub fn run()
{
    let x=5;
    println!{"Now value of x is {x}"};
    let x=x+1;
    println!{"Now value of x is {x}"};
    {
        let x=x*3;
        println!{"Now value of x is {x}"};
    }
    println!{"Now value of x is {x}"};
}