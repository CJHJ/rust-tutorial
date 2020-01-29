mod christmas;
mod fibonacci;
mod temperature;

use std::io;

fn main() {
    let mut selection = String::new();

    println!("Choose your program (temperature, fibonacci, christmas).");
    io::stdin()
        .read_line(&mut selection)
        .expect("Invalid input");

    let selection = selection.trim();

    if selection == "temperature" {
        temperature::temperature(); // Temperature program
    } else if selection == "fibonacci" {
        fibonacci::fibonacci(); // Fibonacci program
    } else if selection == "christmas" {
        christmas::christmas(); // Christmas program
    } else {
        println!("Invalid selection");
    }
}
