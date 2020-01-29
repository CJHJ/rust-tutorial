// Convert temperature program
use std::io;

pub fn temperature() {
    let mut convert_to = String::new();
    let mut temp = String::new();

    println!("Convert to (F or C, origin temperature will be the opposite).");
    io::stdin().read_line(&mut convert_to).expect("Failed to read line");

    println!("Enter the temperature.");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let temp: f32 = temp.trim().parse().expect("Invalid input!");

    let converted = temperature_convert(temp, convert_to);
    println!("Result: {}", converted);
}

fn temperature_convert(t: f32, to_type: String) -> f32 {
    if to_type.trim() == "F" {
        println!("Converting to F...");
        t * 9.0 / 5.0 + 32.0
    } else {
        println!("Converting to C...");
        (t - 32.0) * 5.0 / 9.0
    }
}
