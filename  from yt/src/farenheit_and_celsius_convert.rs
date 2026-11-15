pub fn run()
{
    let farenheit = 98.6;
    let celsius = (farenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F is {}°C", farenheit, celsius);
    let celsius = 37.0;
    let farenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{}°C is {}°F", celsius, farenheit);
}