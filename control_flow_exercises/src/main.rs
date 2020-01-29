mod temperature;
mod fibonacci;

use std::io;

fn main() {
    let mut selection = String::new();

    println!("Choose your program (temperature, fibonacci).");
    io::stdin().read_line(&mut selection).expect("Invalid input");

    if selection.trim() == "temperature" {
        temperature::temperature(); // Temperature program
    } else if selection.trim() == "fibonacci" {
        fibonacci::fibonacci(); // Fibonacci program
    } else {
        println!("Invalid selection");
    }
}