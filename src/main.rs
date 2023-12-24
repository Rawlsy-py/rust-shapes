mod circle;
mod rectangle;
mod square;
mod triangle;

use std::io;

fn main() {
    println!("Welcome to Rust Shapes Calculator");
    println!("What Shape would you like to calculate the area of?");

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let selection = selection.trim().to_lowercase();

    if selection == "circle" {
        println!("Enter the radius of the circle:");

        let mut radius_str = String::new();
        io::stdin()
            .read_line(&mut radius_str)
            .expect("Failed to read line");

        let radius: f64 = match radius_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        };

        let area = circle::calculate_area(radius);
        println!("Area of the circle: {}", area);
    } else if selection == "rectangle" {
        println!("Enter the length and width of the rectangle, separated by a space:");

        let mut dimensions = String::new();
        io::stdin()
            .read_line(&mut dimensions)
            .expect("Failed to read line");

        let parts: Vec<&str> = dimensions.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter two numbers separated by a space.");
            return;
        }

        let length: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid numbers");
                return;
            }
        };

        let width: f64 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid numbers");
                return;
            }
        };

        let area = rectangle::calculate_area(length, width);
        println!("Area of the rectangle: {}", area);
    } else if selection == "square" {
        println!("Enter the length of one of the squares sides:");

        let mut sq_length = String::new();
        io::stdin()
            .read_line(&mut sq_length)
            .expect("Failed to read line");

        let sq_length: f64 = match sq_length.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        };

        let area = square::calculate_area(sq_length);
        println!("Area of the square: {}", area);
    } else if selection == "triangle" {
        println!("Enter the base and height of the triangle, separated by a space:");

        let mut dimensions = String::new();
        io::stdin()
            .read_line(&mut dimensions)
            .expect("Failed to read line");

        let parts: Vec<&str> = dimensions.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter two numbers separated by a space.");
            return;
        }

        let height: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid numbers");
                return;
            }
        };

        let base: f64 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid numbers");
                return;
            }
        };

        let area = triangle::calculate_area(height, base);
        println!("Area of the rectangle: {}", area);
    } else {
        println!(
            "Shape not recognized. Please enter 'Circle', 'Rectangle', 'Triangle' or 'Square'."
        );
    }
    println!("Thank you, have a nice day!");
}
