fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    let z = {
        let x = 3;
        x + 1
    };

    println!("The value is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}, five is {}", z, five());
}

fn five() -> i32 {
    5
}
