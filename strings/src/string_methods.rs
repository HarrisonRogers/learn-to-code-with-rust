use std::io;

pub fn make_money(text: &mut String) {
    text.push_str("$$$");
}

pub fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

pub fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect()
}

pub fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();

    println!("What is your first name?");
    io::stdin()
        .read_line(&mut first_name)
        .expect("No first name found");

    println!("What is your last name?");
    io::stdin()
        .read_line(&mut last_name)
        .expect("No last name found");

    format!("{} {}", first_name.trim(), last_name.trim())
}
