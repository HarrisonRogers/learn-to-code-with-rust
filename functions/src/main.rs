// Declaring functions and parameters
// fn main() {
//     open_store(9, "Queenstown");
//     bake_pizza();
//     swim_in_profit();
//     swim_in_profit();
//     swim_in_profit();
// }

// fn open_store(time: i32, neighboorhood: &str) {
//     println!("Opening pizza store at {}am, {}", time, neighboorhood);
// }

// fn bake_pizza() {
//     println!("Baking pizza");
// }

// fn swim_in_profit() {
//     println!("So much cash yo");
// }

// Return types
// fn main() {
//     let square = square(3);
//     println!("{square}");
// }

// fn square(number: i32) -> i32 {
//     number * number
// }

// Unit return types
// fn main() {
//     let mystery = mystery();
// }

// fn mystery() {}

// Blocks in functions
// fn main() {
//     let multiplier = 5;

//     let calc = {
//         let value = 3 + 2;
//         value * multiplier
//     };

//     println!("{calc}");
// }

/*
Define an apply_to_jobs function that accepts a
'number' parameter (an i32) and a 'title' parameter
(a string). It should print out the string:
"I'm applying to {number} {title} jobs".

Example:
apply_to_jobs(35, "Rust Developer")
-> "I'm applying to 35 Rust Developer jobs"

Define an is_even function that accepts a 'number'
parameter (an i32). The function should return a true
if the number is even and a false if the number is
odd.
Examples:
is_even(8) -> true
is_even(9) -> false

Define an alphabets function that accepts a 'text'
parameter (an &str). The function should return a
tuple of two Booleans. The first Boolean should check
if the text contains the letter 'a'. The second
Boolean should check if the text contains the letter
'z'. You can use the 'contains' method to check if a
string contains a specific character. See the documentation:
https://doc.rust-lang.org/std/primitive.str.html#method.contains

Examples:
println!("{:?}", alphabets("aardvark")); -> (true, false)
println!("{:?}", alphabets("zoology"));  -> (false, true)
println!("{:?}", alphabets("zebra"));    -> (true, true)
*/

fn main() {
    apply_to_jobs(35, "Rust Developer");
    let is_even = is_even(4);
    let alphabets = alphabets("iuahfiauefnaiuefnaeiufz");

    println!("Is even {}", is_even);
    println!(
        "Contains the letter a {}, Contains the letter z {}",
        alphabets.0, alphabets.1
    );
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs")
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("z"))
}
