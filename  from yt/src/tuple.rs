pub fn run()
{
    let tuple = (1_000,true,"String",'C','😊');
    let(a,b,c,d,e) = tuple;
    println!("a is {a}\nb is {b}\nc is {c}\nd is {d}\ne is {e}");
    let x=(1_000,true,"String",'C',2.64532173);
    println!("a is {}\nb is {}\nc is {}\nd is {}\ne is {}",x.0,x.1,x.2,x.3,x.4);
}