// If statement
// fn main() {
//     let value = true;
//     if value {
//         println!("Beans")
//     } else {
//         println!("No Beans :(")
//     }
// }

// Assigning result of if statement
// fn main() {
//     let season = "spring";

//     if season == "summer" {
//         println!("Its hot outside");
//     } else if season == "winter" {
//         println!("GOD DAMN! its cold");
//     } else if season == "spring" {
//         println!("It keeps on raining");
//     } else if season == "autumn" {
//         println!("Leaves be falling");
//     }
// }

// Assigning a variable to if statement
// fn even_or_odd(number: i32) {
//     let result = if number % 2 == 0 { "even" } else { "odd" };
//     println!("The number is {result}");
// }

// fn main() {
//     even_or_odd(16);
// }

// Match statement
// fn main() {
//     let evaluation = true;

//     let value = match evaluation {
//         true => 20,
//         false => 10,
//     };
//     println!("{value}")
// }

// Underscore in a Match Arm
// fn main() {
//     let season = "winter";

//     match season {
//         "summer" => println!("Its warm"),
//         "winter" => println!("cold asf!"),
//         _ => println!("Others"),
//     }
// }

// Match statement with multiple values and conditionals
// fn main() {
//     let number = 1474;

//     match number {
//         // 2 | 4 | 6 | 8 => println!("Number is even"),
//         // 1 | 3 | 5 => println!("Number is odd"),
//         // _ => println!("Beans asf"),
//         value if value % 2 == 0 => println!("{value} is an even number"),
//         value if value % 2 != 0 => println!("{value} is an odd number"),
//         _ => unreachable!(),
//     }
// }

// Iteration
// fn main() {
//     let mut seconds = 21;

//     // Standard loop
//     // loop {
//     //     if seconds <= 0 {
//     //         println!("BLAST OFF!!");
//     //         break; // stops the loop
//     //     }

//     //     if seconds % 2 == 0 {
//     //         println!("{seconds} seconds (even number), skipping 3 seconds...");
//     //         seconds -= 3;
//     //         continue;
//     //     }

//     //     println!("{seconds}s to blast off");
//     //     seconds -= 1; // seconds = seconds - 1
//     // }

//     while seconds > 0 {
//         if seconds % 2 == 0 {
//             println!("{seconds} seconds (even number), skipping 3 seconds...");
//             seconds -= 3;
//             continue;
//         }

//         println!("{seconds}s to blast off");
//         seconds -= 1; // seconds = seconds - 1
//     }

//     println!("Blast off !!");
// }

// Recursion
// fn countdown(seconds: i32) {
//     if seconds == 0 {
//         println!("Blast off!!")
//     } else {
//         println!("{seconds} to blast off..");
//         countdown(seconds - 1);
//     }
// }

// fn main() {
//     countdown(5);
// }

/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.

Refactor the function above to use the `match` statement
instead of if, else if, and else.

Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.

Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.

The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.

Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn color_to_number(color: &str) -> i8 {
    // if color == "red" {
    //     1
    // } else if color == "green" {
    //     2
    // } else if color == "blue" {
    //     3
    // } else {
    //     0
    // }

    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial(number: i32) -> i32 {
    let mut count = number;
    let mut product = 1;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product
}

fn main() {
    println!("{}", factorial(5));

    println!("{}", color_to_number("red"))
}
