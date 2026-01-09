// Scope and ownership
// fn main() {
//     let age = 33;
//     let is_handsome = true;

//     println!("{age}");
//     println!("{is_handsome}");

//     // Age variable exists here
//     // Age variable goes out of scope here
// }

// Copy trait
// fn main() {
//     let time = 2025;
//     let year = time;

//     println!("{year}");
//     println!("{time}");
// }

// String type
// fn main() {
//     let food = "pasta";
//     let text = String::new();

//     let beans = String::from("Beans");
// }

// push_str method
// fn main() {
//     let mut name = String::from("Harrison");
//     println!("{name}");

//     name.push_str(" Rogers");
//     println!("{name}");
// }

// Move & ownership
// fn main() {
//     let person = String::from("Harrison");
//     let genius = person;

//     // println("{person}");
//     println!("{genius}");
// }

// Drop function
// fn main() {
//     let person = String::from("Harrison");

//     // Drops the person variable out of the heap
//     drop(person);

//     // Cannot be used after
// }

// clone method
// fn main() {
//     let person = String::from("Harrison");
//     let genius = person.clone();

//     println!("{person}");
// }

// References and borrowing
// fn main() {
//     let beans = String::from("Beans");
//     let my_beans = &beans;

//     let number = 54;
//     let my_number = &number;
//     let my_number_reference = my_number;

//     println!("{my_number_reference}");
//     println!("{my_number}");
// }

// Dereference operator
// fn main() {
//     let beans = String::from("Beans");
//     let my_beans = &beans;

//     println!("{}", *my_beans);
// }

// All the strings
// fn main() {
//     /*
//     String - A dynamic piece of text stored on the heap at runtime

//     &String ("ref String") - A reference to a heap string

//     str - A hardcoded, read-only piece of text encoded in the binary

//     &str ("ref str") - A reference to the text in the memory that has loaded the binary file
//     */
//     let ice_cream = "Chocolate";
//     println!("{}", ice_cream)
// }

// Copy trait with references
// fn main() {
//     let ice_cream = "Chocolate";
//     let dessert = ice_cream;

//     println!("{ice_cream} {dessert}");
// }

// Ownership params
// fn print_my_integer(value: i32) {
//     println!("Your value is {}", value)
// }

// fn print_my_string(string: String) {
//     println!("{}", string);
// }
// fn main() {
//     let apples = 6;
//     let mangoes = String::from("Mango");

//     print_my_integer(apples);
//     print_my_string(mangoes);

//     println!("{}", apples);
// }

// Mutable params
// fn main() {
//     let burger = String::from("Burger");
//     println!("{}", &burger);

//     add_fries(burger);
// }

// fn add_fries(mut meal: String) {
//     meal.push_str(" and Fries");
//     println!("{}", meal);
// }

// Return values 1
// fn main() {
//     let cake = bake_cake();
//     println!("My cake is a {}", cake)
// }

// fn bake_cake() -> String {
//     String::from("Cheese cake")
// }

// Return types 2
// fn main() {
//     let mut current_meal = String::new();
//     current_meal = add_flour(current_meal);

//     println!("{}", current_meal);
// }

// fn add_flour(mut meal: String) -> String {
//     meal.push_str("Add flour");
//     meal
// }

/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.

The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.

In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.

Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

// fn main() {
//     let is_concert = true;
//     let is_event = is_concert;

//     println!("{}", is_concert);
//     println!("{}", is_event);

//     // this won't work - sushi ownership has been passed to dinner
//     // let sushi = String::from("Salmon");
//     // let dinner = sushi;

//     // println!("{}", sushi);
//     // println!("{}", dinner);

//     // This will work - because it is a heap string an it is always referenced
//     // let sushi = "Salmon";
//     // let dinner = sushi;

//     // println!("{}", sushi);
//     // println!("{}", dinner);

//     let sushi = String::from("Salmon");
//     let result = eat_meal(sushi);
//     println!("You have eaten you meal {}", result);
//     // What happens to sushi ownership is it gets passed onto the eat_meal function because we aren't requesting a reference
// }

// fn eat_meal(mut meal: String) -> String {
//     meal.clear();
//     meal
// }

// --- Next part

// Immutable and mutable references
// fn main() {
//     let mut current_meal = String::new();
//     add_flour(&mut current_meal);
//     show_my_meal(&current_meal);
// }

// fn add_flour(meal: &mut String) {
//     meal.push_str("Add flour");
// }

// fn show_my_meal(meal: &String) {
//     println!("Meal steps {}", meal);
// }

// Multiple mutable references & restrictions
// fn main() {
//     let mut car = String::from("Red");
//     let ref1 = &mut car;
//     let ref2 = &car;

//     println!("{}", ref2);
// }

// Ownership with immutable and mutable references
// fn main() {
//     let mut coffee = String::from("Flat white");
//     // Doesn't work
//     // let a = &mut coffee;
//     // let b = a;
//     // println!("{} {}", a, b);

//     // Does work
//     let a = &mut coffee;
//     println!("{}", a);
//     let b = a;
//     println!("{}", b);
// }

// Dangling reference
// fn main() {
//     let city = create_city();
//     println!("{}", city)
// }

// fn create_city() -> String {
//     String::from("Queenstown")
// }

// Ownership of arrays and tuples
// fn main() {
//     let registrations = (true, true, false);
//     let first = registrations.0;
//     println!("{} and {:#?}", first, registrations);

//     let languages = [String::from("Rust"), String::from("TypeScript")];
//     let first = &languages[0];
//     println!("{} and {:#?}", first, languages);
// }

/*
Let's model a road trip!

Define a `start_trip` function that creates and returns
a String of "The plan is..."

Invoke the `start_trip` function in `main` and save its
return value to a `trip` variable.

We want to pass the String to three separate functions
that will mutate the String without transferring ownership.

Define a `visit_philadelphia` function that concatenates
the text "Philadephia" to the end of the String. Invoke
the function in `main`. Then, invoke `push_str` on the String
to concatenate the content " and " to the end. Mak sure to
include the spaces.

Define a `visit_new_york` function that concatenates the
text "New York" to the end of the String. Invoke the function
in `main`. Repeat the previous logic to concatenate " and "
to the end of the String.

Define a `visit_boston` function that concatenates the
text "Boston." to the end of the String. Invoke the function
in `main`. Concatenate a period to the end of the
String/sentence.

Define a `show_itinerary` function that will print out
the final version of the String. Find a way to do so
without transferring ownership.

Invoke `show_itinerary`. The final output should be:

"The plan is...Philadelphia and New York and Boston."
*/

fn main() {
    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    trip.push_str(".");

    show_itinerary(&trip);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadelphia")
}

fn visit_new_york(trip: &mut String) {
    trip.push_str("New York")
}

fn visit_boston(trip: &mut String) {
    trip.push_str("Boston");
}

fn show_itinerary(trip: &String) {
    println!("{}", trip)
}
