use std::io::stdin;

fn main() {
    let mut fahrenheit = String::new();
    println!("Convert Fahrenheit to Celsius");
    println!("Input value for Fahrenheit");
    stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit
        .trim()
        .parse()
        .expect("Please enter a number");

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("Value of Celsius: {celsius}");

}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}