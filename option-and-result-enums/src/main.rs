// Option Enum
// fn main() {
//     let a = Option::Some(String::from("Beans"));
//     let b = Option::Some(4.3);
//     let c = Option::Some(2);

//     let a: Option<i32> = Option::Some(5);
//     let a = Option::<i16>::Some(1);

//     let d: Option<&str> = Option::None;
// }

// Real example of option enum
// fn main() {
//     let musical_instruments = [
//         String::from("Guitar"),
//         String::from("Drums"),
//         String::from("Piano"),
//     ];

//     let bass = musical_instruments.get(2);
//     println!("{:?}", bass);

//     let invalid_instrument = musical_instruments.get(100);
//     println!("{:?}", invalid_instrument);
// }

// --- Unwrap and expect
// fn main() {
//     let musical_instruments = [
//         String::from("Guitar"),
//         String::from("Drums"),
//         String::from("Piano"),
//     ];

//     let bass = musical_instruments.get(2);
//     println!("{:?}", bass);
//     let valid_instrument = bass.expect("Unable to retrieve instrument");
//     println!("{}", valid_instrument);

//     let invalid_instrument = musical_instruments.get(100);
//     println!("{:?}", invalid_instrument);
//     // invalid_instrument.unwrap();
// }

// --- Match statement
// fn main() {
//     let musical_instruments = [
//         String::from("Guitar"),
//         String::from("Drums"),
//         String::from("Piano"),
//     ];

//     let bass = musical_instruments.get(2);
//     play(bass);

//     let invalid_instrument = musical_instruments.get(100);
//     play(invalid_instrument);
// }

// fn play(instrument_option: Option<&String>) {
//     match instrument_option {
//         Option::Some(instrument) => println!("Playing the {}", instrument),
//         Option::None => println!("You are the vocalist"),
//     }
// }

// --- Returning an option enum from a function
// fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
//     if item_is_in_system && item_is_in_stock {
//         Option::Some(true)
//     } else if item_is_in_system {
//         Option::Some(false)
//     } else {
//         Option::None
//     }
// }

// fn main() {
//     let availability = is_item_in_stock(true, false);

//     // match availability {
//     //     Option::Some(item) => println!("Item is available: {}", item),
//     //     Option::None => println!("Your item doesn't exist in our system"),
//     // }
//     match availability {
//         Option::Some(true) => println!("Yes, the item is available"),
//         Option::Some(false) => println!("No, your item is no available"),
//         Option::None => println!("Your item doesn't exist in our system"),
//     }
// }

// --- Top level option variants
// fn main() {
//     let item_is_available = is_item_in_stock();

//     match item_is_available {
//         Some(true) => println!("Yes, the item is in stock and available"),
//         Some(false) => println!("No, the item is not in stock"),
//         None => println!("We're unable to find the item that you're looking for"),
//     }
// }

// fn is_item_in_stock() -> Option<bool> {
//     let item_exists_in_catalog = true;
//     let item_is_in_stock = false;

//     if item_exists_in_catalog && item_is_in_stock {
//         Some(true)
//     } else if item_exists_in_catalog && !item_is_in_stock {
//         Some(false)
//     } else {
//         None
//     }
// }

// -- unwrap_or method
// fn main() {
//     let present_value: Option<i32> = Some(13);
//     let missing_value: Option<i32> = None;

//     println!("{}", present_value.unwrap_or(0));
//     println!("{}", missing_value.unwrap_or(0));
// }

// --- Building Option from scratch
// #[derive(Debug, Copy, Clone)]
// enum MyOption {
//     Some(i32),
//     None,
// }

// impl MyOption {
//     fn unwrap(self) -> i32 {
//         match self {
//             MyOption::Some(value) => value,
//             MyOption::None => panic!("Oh no bro"),
//         }
//     }

//     fn unwrap_or(self, or_value: i32) -> i32 {
//         match self {
//             MyOption::Some(value) => value,
//             MyOption::None => or_value,
//         }
//     }
// }

// fn main() {
//     let some_option = MyOption::Some(43);
//     println!("{}", some_option.unwrap());

//     let none_option = MyOption::None;
//     // println!("{}", none_option.unwrap());
//     println!("{}", none_option.unwrap_or(3));
// }

// --- Result enum
// fn main() {
//     let ok: Result<i8, &str> = Ok(5);
//     println!("{:?}", ok);
//     let disaster: Result<i8, &str> = Err("Something went wrong");
//     println!("{:?}", disaster);
// }

// -- Real example of a Result enum
// fn main() {
//     let text = "50";
//     let text_as_number = text.parse::<i8>();
//     println!("{:?}", text_as_number);

//     let text = "alabama";
//     let text_as_number = text.parse::<i8>();
//     println!("{:?}", text_as_number);
// }

// --- Returning a result enum from a function && methods
// fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err(String::from("You can't divide by zero"))
//     } else {
//         Ok(numerator / denominator)
//     }
// }

// fn main() {
//     let result = divide(10.0, 0.0);
//     println!("{:?}", result.is_ok());
//     println!("{:?}", result.is_err());

//     // match result {
//     //     Ok(calculation) => println!("{:?}", calculation),
//     //     Err(message) => println!("Error: {}", message),
//     // }
// }

// --- Nuance of unwrap on result enum
// fn operation(great_success: bool) -> Result<&'static str, &'static str> {
//     if great_success {
//         Ok("Success")
//     } else {
//         Err("Error")
//     }
// }

// fn main() {
//     let my_result = operation(true);

//     let content = match my_result {
//         Ok(message) => message,
//         Err(message) => message,
//     };

//     println!("{}", my_result.unwrap())
// }

// --- The while let construct
// fn main() {
//     let mut sauces = vec!["Mayo", "Tomato Sauce", "Aioli"];

//     while let Some(sauce) = sauces.pop() {
//         println!("The next sauce is {}", sauce);
//     }
// }

/*
Define a Food struct with a single `name` field
set to a String. Derive a Debug implementation.

Define a Restaurant struct with a `reservations` field
set to a u32 and a `has_mice_infestation` field set to
a bool. Derive a Debug implementation.

Define a `chef_special` method on the Restaurant.
The method will return the restaurant's famous
dish. It should return an Option containing a Food
struct.

If the restaurant has a mice infestation, return the
None variant. There is no chef special!

If the restaurant has less than 12 reservations, return
a Food instance with a name of "Uni Sashimi" wrapped in
the Some variant. If it has 12 or more reservations,
return a Food instance with a name of "Strip Steak"
instead, also wrapped in the Some variant.

Define a `deliver_burger` method on the Restaurant.
It should accept an `address` string slice; it will
represent where to deliver the order. It should
return a Result type where the Ok variant holds a Food
struct and the Err variant holds a String.

If the restaurant has a mice infestation, return the
Err variant containing a String of "Sorry, we have a
mice problem".

If the user's address is an empty string, return the Err
variant with a String of "No delivery address specified".
HINT: You can use the `is_empty` method on a string to check
if it has 0 characters.
https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty

Otherwise, the delivery is good to go! Return the Ok
variant containing a Food struct with a `name` of "Burger".

In the `main` function, create a `Restaurant` instance
with 11 reservations and a mice infestation.

Invoke the `chef_special` method and print out its return
value. It should be the None variant.

Invoke the `deliver_burger` method with an argument of "123
Elm Street" and print out its return value. It should be
the Err variant.

Create another `Restaurant` instance with 15 reservations
and no mice infestation.

Invoke the `chef_special` method and print out its return
 value. It should be the Some variant with a "Strip Steak".

Invoke the `deliver_burger` method with an argument of an
empty address. Print out its return value. It should be the
Err variant.

Invoke the `deliver_burger` method again with an argument
of a valid address. Print out its return value. It should
be the Ok variant nesting a Food struct with a `name` of
"Burger".
*/

#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            None
        } else if self.reservations > 12 {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Strip Steak"),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err(String::from("Sorry, we have a mice problem"))
        } else if address.is_empty() {
            Err(String::from("No delivery address specified"))
        } else {
            Ok(Food {
                name: String::from("Burger"),
            })
        }
    }
}

fn main() {
    let restaurant = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    println!("{:?}", restaurant.chef_special());
    println!("{:?}", restaurant.deliver_burger("121 beans place"));

    let other_restaurant = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    println!(
        "{:?}",
        other_restaurant.chef_special().expect("No fricken food")
    );
    println!("{:?}", other_restaurant.deliver_burger(""));
    println!("{:?}", other_restaurant.deliver_burger("Address yo"))
}
