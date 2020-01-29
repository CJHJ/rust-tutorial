fn main() {
    // If - else conditions
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // If - else conditions with expression
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // Loop
    let mut i = 0;
    let result = loop {
        print!("{} ", i);

        i = add(i, 1);
        if i > 5 {
            println!();
            break i * 5;
        }
    };
    println!("The result after loop: {}", result);

    // While loop
    let mut number = 3;
    while number != 0 {
        print!("{}! ", number);

        number -= 1;
    }
    println!();
    println!("LIFTOFF!!!");

    // Array loop
    let a = [10, 20, 30, 40, 50];

    print!("The value of array is: ");
    for element in a.iter() {
        print!("{} ", element);
    }

    // Array loop with range
    print!("\nThe value of range is: ");
    for number in (1..4).rev() {
        print!("{} ", number);
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
