pub fn christmas() {
    let mut i = 1;

    while i < 12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            number_order(i)
        );
        println!("A partridge in a pear tree\n");

        i += 1;
    }
}

fn number_order(i: u32) -> String {
    if i == 1 {
        "first".to_string()
    } else if i == 2 {
        "second".to_string()
    } else if i == 3 {
        "third".to_string()
    } else if i == 4 {
        "fourth".to_string()
    } else if i == 5 {
        "fifth".to_string()
    } else if i == 6 {
        "sixth".to_string()
    } else if i == 7 {
        "seventh".to_string()
    } else if i == 8 {
        "eighth".to_string()
    } else if i == 9 {
        "ninth".to_string()
    } else if i == 10 {
        "tenth".to_string()
    } else if i == 11 {
        "eleventh".to_string()
    } else if i == 12 {
        "twelfth".to_string()
    } else {
        "invalid".to_string()
    }
}
