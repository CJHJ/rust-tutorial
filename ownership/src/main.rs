fn main() {
    // Move
    let s1 = String::from("hello");
    let s2 = s1; // s1 will be freed, data from s1 will be copied to s2.
    println!("s2 = {}", s2);

    // Clone - deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Primitive (type with known size during compile time) save it's value in a variable in a stack, meaning moving is not required
    // You can still use the original variable
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // References and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // '&' create references (doesn't own the value)
    println!("The length of '{}' is {}.", s1, len);

    // Borrowing and modifying
    let mut s1 = String::from("hello");
    append_world(&mut s1);
    println!("s1 becomes {}", s1);

    // Cannot combine mutable and immutable references
    let mut s2 = String::from("Test");

    let r1 = &s2;
    let r2 = &s2;
    println!("{} and {} is immutable references", r1, r2);

    let r3 = &mut s2; // Can do this because of scope
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len() // We can call functions, but we cannot modify something on immutable reference
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}
