fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s // Will result in a dangling reference because 's' goes out of scope
    s // OK
}