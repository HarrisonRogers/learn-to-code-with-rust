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

// Writing to file & asking user for input & Reading the files content
// use std::io::{Read, stdin};
// use std::{fs::File, process};

// fn main() {
//     println!("Please enter the name of the file you'd like to read");
//     let mut input = String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requested_file {
//         eprintln!("Something went wrong collecting user input: {}", error);
//         process::exit(1)
//     }

//     let mut file = match File::open(input.trim()) {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!("No file here bruh: {}", error);
//             process::exit(1);
//         }
//     };

//     let mut content = String::new();
//     let read_operation = file.read_to_string(&mut content);

//     if let Err(error) = read_operation {
//         eprintln!("Something went wrong reading the file: {}", error);
//         process::exit(1)
//     }

//     println!("{}", content);
// }

// --- Delegating errors & ? operator
use std::fs::File;
use std::io::{self, Read, stdin};

fn main() {
    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{}", contents),
        Err(error) => {
            eprintln!("There was an error: {}", error);
        }
    }
}

// fn read_file() -> Result<String, io::Error> {
//     println!("Please enter the name of the file you'd like to read");
//     let mut input = String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requested_file {
//         return Err(error);
//     }

//     let mut file = match File::open(input.trim()) {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };

//     let mut content = String::new();
//     let read_operation = file.read_to_string(&mut content);

//     if let Err(error) = read_operation {
//         return Err(error);
//     }

//     Ok(content)
// }

// --- ? operator
fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let mut content = String::new();
    File::open(input.trim())?.read_to_string(&mut content)?;

    Ok(content)
}
