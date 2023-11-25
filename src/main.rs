mod circle;

use std::io;


fn main() {
    // Welcome Message
    println!("Welcome to Rust Shapes Calculator");

    println!("What Shape would you like to calculate the area of?");

    let mut selection: String = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    if selection == "Circle" {
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
    }
    else {
        println!("Thank you, Have a nice day!")
    }
}