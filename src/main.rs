mod circle;

use std::io;


fn main() {
    // Welcome Message
    println!("Welcome to Rust Shapes Calculator");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let number: f64 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let radius = number;

    let area = circle::calculate_area(radius);
    println!("Area of the circle: {}", area);
}