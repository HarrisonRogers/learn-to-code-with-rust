// use std::io;

// fn main() {
//     let mut name = String::new();
//     println!("What is your name?");
//     match io::stdin().read_line(&mut name) {
//         Ok(_) => println!("Hello, {}!", name.trim()),
//         Err(error) => println!("There was an error: {}", error),
//     }
// }

// --- Project

use crate::string_methods::get_identity;

mod string_methods;
fn main() {
    let mut my_text = String::from("Harrison");
    string_methods::make_money(&mut my_text);
    println!("Add $$$: {}", my_text);

    let capatilzed = string_methods::trim_and_capitalize(&my_text);
    println!("Capatilsed: {}", capatilzed);

    let string_slice = "Beans!Fork!Toast!Butter";
    let result = string_methods::elements(string_slice);
    println!("What do we have here? {:?}", result);

    let name = get_identity();
    println!("Your name is {}", name);
}
