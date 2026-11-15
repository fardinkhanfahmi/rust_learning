use std::io;
pub fn run()
{
    let mut farenheit = String::new();
    io::stdin().read_line(&mut farenheit).unwrap();
    let farenheit:f32 = farenheit.trim().parse().unwrap();
    let celsius = (farenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F is {}°C", farenheit, celsius);
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).unwrap();
    let celsius:f32 = celsius.trim().parse().unwrap();
    let farenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{}°C is {}°F", celsius, farenheit);
}