// fn main() {
//     let value: i32 = -15;
//     println!("{}", value.abs());

//     let empty_space: &str = "       uaifeeaf      ";
//     println!("{}", empty_space.trim());

//     println!("{}", value.pow(2));
//     println!("{}", value.pow(5));
// }

// Integers
// fn main() {
//     let value: i32 = -15;
//     println!("{}", value.abs());

//     let empty_space: &str = "       uaifeeaf      ";
//     println!("{}", empty_space.trim());

//     println!("{}", value.pow(2));
//     println!("{}", value.pow(5));
// }

// Floats
// fn main() {
//     let pi: f64 = 3.14159;
//     println!("The current value is {pi}");

//     println!("{}", pi.floor());
// }

// fn main() {
//     let pi: f64 = 3.141598792529835;
//     println!("The current value of pi is {:.3}", pi);
// }

// Casting types
// fn main() {
//     let miles_away: i32 = 50;
//     let miles_away_i8 = miles_away as i8;
//     let miles_away_u8 = miles_away as u8;

//     let miles_away = 100.29839248;
//     let miles_away_f32 = miles_away as f32;
//     let miles_away_int = miles_away as i32;

//     println!("{miles_away_int}")
// }

// Math operations
// fn main() {
//     let addition = 5 + 2;
//     let subtraction = 10 - 3;
//     let multiplication = 3 * 4;

//     println!("Additon: {addition}, subtraction: {subtraction}, multiplication: {multiplication}");

//     let floor_division = 5 / 3;
//     println!("{floor_division}");

//     let decimal_division = 5.0 / 3.0;
//     println!("{decimal_division}");

//     let remainder = 10 % 5;
//     println!("{remainder}");
// }

// Augmented assignment operators
// fn main() {
//     let mut year = 2025;
//     year += 1;
//     println!("The new year is {year}");

//     year -= 5;
//     println!("The new year is {year}");

//     year *= 2;
//     println!("The year is {year}");

//     year /= 4;
//     println!("The new year is {year}");
// }

// Booleans
// fn main() {
//     let is_handsome: bool = true;
//     let is_goofy_ahh: bool = true;

//     if is_handsome {
//         println!("Harrison is so handsome")
//     } else {
//         println!("Eww harrison is yuck")
//     }

//     if is_goofy_ahh {
//         println!("67 goofy ahh")
//     } else {
//         println!("whats 67?")
//     }

//     let age = 19;
//     let is_young = age < 25;
//     print!("Am I young {is_young}");
// }

// Character types
// fn main() {
//     let first_inital = 'H';
//     let emoji = '🤣';

//     println!("{} {}", first_inital.is_alphabetic(), emoji.is_alphabetic());
//     println!("{} {}", first_inital.is_uppercase(), emoji.is_uppercase());
// }

// Array types
// fn main() {
//     let numbers: [i32; 6] = [2, 5, 2, 4, 9, 7498];

//     let fruits = ["apple", "beans", "orange"];
//     println!("Length {}", fruits.len());

//     let currency_rates = [5.4];
// }

// Reading and writing array types
// fn main() {
//     let mut fruits = ["bananas", "pear", "oranges", "apples"];
//     println!("Crazy fruit is: {}", fruits[2]);

//     fruits[2] = "mango";

//     println!("Fruit at index 2: {}", fruits[2]);
// }

// Format specifiers for outputting
// fn main() {
//     let seasons = ["Summer", "spring", "winter", "autum"];

//     println!("{}", 5);
//     println!("{}", true);
//     println!("{}", 5.4);
//     // Must add debug to see output
//     println!("{:#?}", seasons);
// }

// Tuple types
// fn main() {
//     let me = ("Harrison", 19, "Software");

//     // let name = me.0;
//     // let age = me.1;
//     // let department = me.2;

//     let (name, age, department) = me;

//     println!("Name: {}, Age: {}, Department: {}", name, age, department);

//     println!("{:#?}", me)
// }

// Range type
// fn main() {
//     // Excludes 31
//     let month_days = 1..31;
//     println!("{:#?}", month_days);

//     // Includes 31
//     let month_days = 1..=31;
//     println!("{:#?}", month_days);
//     for day in month_days {
//         println!("{}", day);
//     }

//     let letters = 'b'..'g';
//     for letter in letters {
//         println!("{}", letter);
//     }

//     let colors = ["red", "green", "blue"];
//     for color in colors {
//         println!("Beans {}", color);
//     }
// }

// Generic types

// use std::ops::Range;
// fn main() {
//     let month_days: Range<i8> = 1..31;
//     let letters: Range<char> = 'a'..'j';

//     println!("{:#?} {:#?}", month_days, letters);
// }

/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.

Cast the i32 to an i16 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() {
    let high_number = 1337;
    let high_number_i16 = high_number as i16;
    println!("{}", high_number_i16);

    let float = 3.475898498;
    println!("{:.3}", float);

    let with_milk = true;
    let with_sugar = false;
    let is_my_type_of_coffee = with_sugar && with_milk;
    let is_acceptable_coffee = with_sugar || with_milk;
    println!("Is it my type? {}", is_my_type_of_coffee);
    println!("Is it acceptable? {}", is_acceptable_coffee);

    let arr: [i8; 4] = [12, 3, 5, 18];
    let tuple = (3, 4.5, true, arr);
    println!("{:?}", tuple);
}
