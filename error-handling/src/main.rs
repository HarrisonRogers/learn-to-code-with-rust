// --- Panic
// fn main() {
//     let a = 1;
//     panic!("Error: oh shit boi")
// }

// --- Process Module
// use std::process;

// fn main() {
//     process::exit(1)
// }

// --- eprinln!()
// fn main() {
//     println!("Beans");

//     // this will segregate the error message if not a failiure (A bit pointless tbh)
//     eprintln!("Error Beans");
// }

// Writing to file
use std::{fs::File, process};

fn main() {
    let file = match File::open("story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("No file here bruh: {}", error);
            process::exit(1);
        }
    };

    println!("{:#?}", file)
}
