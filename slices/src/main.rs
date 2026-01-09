// String slices & literals
// fn main() {
//     // let action_hero = String::from("The Rock");
//     let action_hero = "The Rock";
//     let first_name = &action_hero[4..8];
//     println!("{}", first_name);

//     let first_name = {
//         let actor = "Brad Pitt";
//         &actor[0..4]
//     };

//     println!("{}", first_name)

//     // let the_part = &action_hero[0..3];
//     // println!("{}", the_part)
// }

// Length of string slice
// fn main() {
//     let food = "🥷🏿";
//     println!("{}", food.len());
//     let pizza_slice = &food[0..6];
//     println!("{}", pizza_slice.len())
// }

// Syntactic shortcuts
// fn main() {
//     let action_hero = String::from("The Rock");

//     // Start at byte 4 and continue till end of String
//     let first_name = &action_hero[4..];
//     println!("{}", first_name);

//     // Start to third
//     let the_part = &action_hero[..3];
//     println!("{}", the_part);

//     let full_name = &action_hero[..];
//     println!("His full name is {}", full_name);
// }

// String slices as function params
// fn main() {
//     let actor = String::from("Tom Cruise");
//     do_acting(&actor);
//     let another = &actor[0..3];
//     do_acting(&another);
// }

// fn do_acting(actor: &str) {
//     // Will work for both because &String becomes &str
//     println!("{} is an actor", actor)
// }

// Array slices
// fn main() {
//     let values = [4, 8, 15, 78, 30, 12];

//     let my_slice = &values[..4];
//     println!("{:?}", my_slice);

//     let my_slice = &values[3..];
//     println!("{:?}", my_slice);

//     let my_slice = &values[..];
//     println!("{:?}", my_slice)
// }

// Deref coersion
// fn main() {
//     let values = [4, 8, 15, 78, 30, 12];

//     let regular_reference = &values;
//     print_length(regular_reference);

//     let slice_of_three = &values[..3];
//     print_length(slice_of_three);
// }

// fn print_length(reference: &[i32]) {
//     println!("{}", reference.len());
// }

// Mutable array slices
// fn main() {
//     let mut integers = [2, 17, 8, 90, 23, 42];
//     let my_slice = &mut integers[3..5];
//     println!("My slice is {:?}", my_slice);

//     my_slice[0] = 100;
//     println!("My slice is {:?}", my_slice);
//     println!("My slice is {:?}", integers);
// }

/*
Define a `cereals` array with 5 heap Strings
  - Cookie Crisp
  - Cinnamon Toast Crunch
  - Frosted Flakes
  - Cocoa Puffs
  - Captain Crunch

Declare a `first_two` variable that extracts a slice
of the first two cereals. Print the slice.

Declare a `mid_three` variable that extracts a slice
of the middle three cereals (Cinnamon Toast Crunch
up to and including Cocoa Puffs). Print the slice.

Declare a `last_three` variable that extracts a slice
of the last three cereals. Print the slice.

Using the `last_three` slice, target the last element
("Captain Crunch") and replace it with "Lucky Charms".
Print the complete `cereals` array.

Declare a `cookie_crisp` variable. Make it a reference
to the "Cookie Crisp" String (in other words, a &String).

Declare a `cookie` variable that extracts a slice of
the text "Cookie" from the String. Print the slice.

Declare a `cocoa_puffs` variable. Make it a reference
to the "Cocoa Puffs" String (in other words, a &String).

Declare a `puffs` variable that extracts a slice of
the text "Puffs" from the String. Print the slice.
*/
fn main() {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &cereals[..2];
    println!("{:?}", first_two);
    let mid_three = &cereals[1..4];
    println!("{:?}", mid_three);
    let last_three = &mut cereals[2..];
    println!("{:?}", last_three);
    last_three[2] = String::from("Lucky Charms");
    println!("New cereals {:?}", cereals);

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..6];
    println!("{}", cookie);

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("{}", puffs)
}
