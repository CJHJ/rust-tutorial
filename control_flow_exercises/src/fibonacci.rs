// Calculate fibonacci number
use std::io;

pub fn fibonacci() {
    let mut n = String::new();

    println!("Enter a number.");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Failed to parse");

    let result = fib(n);
    println!("Result: {}", result);
}

fn fib(n: usize) -> u64 {
    let mut i = 2;
    let mut arr: [u64; 2] = [0, 1];

    if n < 2 {
        return arr[n];
    }

    while i <= n {
        let temp = arr[0] + arr[1];
        arr[0] = arr[1];
        arr[1] = temp;

        i += 1;
    }

    return arr[1];
}
