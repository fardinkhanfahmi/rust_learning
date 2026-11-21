pub fn run()
{
    let s=String::from("Hello World");
    let res=find_first_word(s);
    println!("Result is {res}");
}
fn find_first_word(input : String) -> usize
{
    let s=input.as_bytes();
    for(i,&item) in s.iter().enumerate()
    {
        if item==b' '{
            return i;
        }
    }
    s.len()
}