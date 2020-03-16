fn main() {
    let mut s = String::from("hello world");
    let first_word = get_first_word(&s[..]);

    let my_string_literal = "hello world";
    let word = get_first_word(&my_string_literal[..]);
    let word = get_first_word(my_string_literal);
    
    println!("{}", first_word);
    println!("{}", word);

    // s.clear(); // Cannot double borrow mutable, also cannot borrow mutable if previously borrowed as immutable

    println!("After clearning: {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        } 
    }

    &s[..]
}