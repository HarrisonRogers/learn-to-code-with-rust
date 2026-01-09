// Displaying structs & instances & accessing & overwriting
// fn main() {
//     struct Coffee {
//         price: f64,
//         name: String,
//         is_hot: bool,
//     }

//     let mut flat_white = Coffee {
//         price: 2.99,
//         name: String::from("Flat White"),
//         is_hot: true,
//     };

//     flat_white.price = 1.99;

//     println!(
//         "{} costs {} and is it hot {}",
//         flat_white.name, flat_white.price, flat_white.is_hot
//     );
// }

// Structs in a function & Shortcuts
// struct Coffee {
//     price: f64,
//     name: String,
//     is_hot: bool,
// }

// fn main() {
//     let name = String::from("Americano");
//     let coffee = make_coffee(name, 3.99, false);

//     println!("{} {} {}", coffee.name, coffee.price, coffee.is_hot);
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// Struct update syntax
// #[derive(Debug)]
// struct Coffee {
//     name: String,
//     price: f64,
//     is_hot: bool,
// }

// fn main() {
//     let mocha = make_coffee(String::from("Mocah"), 2.99, true);

//     let caramel_latte = Coffee {
//         name: String::from("Caramel Latte"),
//         ..mocha
//     };
//     println!("{:?}", caramel_latte)
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// Passing structs into a function
// struct Coffee {
//     name: String,
//     price: f64,
//     is_hot: bool,
// }

// fn main() {
//     let mut mocha = make_coffee(String::from("Mocha"), 1.99, true);
//     drink_coffee(&mut mocha);
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// // Mutable
// // fn drink_coffee(mut coffee: Coffee) {
// //     println!("Drinking my delicious {}", coffee.name);
// //     coffee.is_hot = false;
// // }
// // Reference
// // fn drink_coffee(coffee: &Coffee) {
// //     println!("Drinking my delicious {}", coffee.name);
// // }
// // Mutable reference
// fn drink_coffee(coffee: &mut Coffee) {
//     println!("Drinking my delicous {}", coffee.name);
//     coffee.is_hot = false;
//     coffee.price = 4.32;
// }

// Methods on structs
// #[derive(Debug)]
// struct DireStraitsSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl DireStraitsSong {
//     fn display_song_info(self) {
//         println!("Title: {}", self.title);
//         println!("Release: {}", self.release_year);
//         println!("Duration: {}s", self.duration_secs);
//     }
// }

// fn main() {
//     let song = DireStraitsSong {
//         title: String::from("Lady Writer"),
//         release_year: 1982,
//         duration_secs: 200,
//     };

//     song.display_song_info
// }

// Self parameter as immutable & mutable references to struct instance
// #[derive(Debug)]
// struct DireStraitsSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// // self
// // mut self
// // &self
// // &mut self

// impl DireStraitsSong {
//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Release: {}", self.release_year);
//         println!("Duration: {}s", self.duration_secs);
//     }

//     // Mutable
//     // fn double_length(mut self) {
//     //     self.duration_secs = self.duration_secs * 2;
//     //     println!("{:?}", self);
//     // }

//     // Mutable reference
//     fn double_length(&mut self) {
//         self.duration_secs = self.duration_secs * 2;
//         println!("{:?}", self);
//     }
// }

// fn main() {
//     let mut song = DireStraitsSong {
//         title: String::from("Lady Writer"),
//         release_year: 1982,
//         duration_secs: 200,
//     };

//     song.display_song_info();
//     song.double_length();
//     song.display_song_info();
// }

// Methods with multiple parameters
// #[derive(Debug)]
// struct DireStraitsSong {
//     title: String,
//     release_year: u32,
//     length: u32,
// }

// impl DireStraitsSong {
//     fn double_length(&mut self) -> u32 {
//         self.length * 2
//     }

//     fn display_song_info(&self) {
//         println!(
//             "Title: {}, Release year: {}, duration: {}s",
//             self.title, self.release_year, self.length
//         )
//     }

//     fn is_longer_then(&self, other: &Self) -> bool {
//         self.length > other.length
//     }
// }

// fn main() {
//     let mut song = DireStraitsSong {
//         title: String::from("Lady Writer"),
//         release_year: 1982,
//         length: 200,
//     };

//     let other_song = DireStraitsSong {
//         title: String::from("Sultans of Swing"),
//         length: 230,
//         release_year: 1985,
//     };

//     song.double_length();
//     println!("{:?}", song.double_length());

//     song.display_song_info();

//     if song.is_longer_then(&other_song) {
//         println!("{} is longer than {}", song.title, other_song.title)
//     } else {
//         println!(
//             "{} is shorter than or equal too {}",
//             song.title, other_song.title
//         )
//     }
// }

// -- Calling methods from other methods
// #[derive(Debug)]
// struct DireStraitsSong {
//     title: String,
//     release_year: u32,
//     length: u32,
// }

// impl DireStraitsSong {
//     fn double_length(&mut self) {
//         self.length = self.length * 2;
//     }

//     fn display_song_info(&self) {
//         println!(
//             "Title: {}, Release year: {}, duration: {}s, Years since release: {}",
//             self.title,
//             self.release_year,
//             self.length,
//             self.years_since_release()
//         )
//     }

//     fn is_longer_then(&self, other: &Self) -> bool {
//         self.length > other.length
//     }

//     fn years_since_release(&self) -> u32 {
//         2025 - self.release_year
//     }
// }

// fn main() {
//     let song = DireStraitsSong {
//         title: String::from("Lady Writer"),
//         release_year: 1982,
//         length: 200,
//     };

//     song.display_song_info();
// }

// -- Associated functions && Multiple impl blocks
// #[derive(Debug)]
// struct DireStraitsSong {
//     title: String,
//     release_year: u32,
//     length: u32,
// }

// // Separating methods from constructor code
// impl DireStraitsSong {
//     fn new(title: String, release_year: u32, length: u32) -> Self {
//         Self {
//             title,
//             release_year,
//             length,
//         }
//     }
// }

// impl DireStraitsSong {
//     fn double_length(&mut self) {
//         self.length = self.length * 2;
//     }

//     fn display_song_info(&self) {
//         println!(
//             "Title: {}, Release year: {}, duration: {}s, Years since release: {}",
//             self.title,
//             self.release_year,
//             self.length,
//             self.years_since_release()
//         )
//     }

//     fn is_longer_then(&self, other: &Self) -> bool {
//         self.length > other.length
//     }

//     fn years_since_release(&self) -> u32 {
//         2025 - self.release_year
//     }
// }

// fn main() {
//     let song = DireStraitsSong {
//         title: String::from("Lady Writer"),
//         release_year: 1982,
//         length: 200,
//     };

//     let new_song = DireStraitsSong::new(String::from("Sultans of Swing"), 1985, 180);

//     println!("{:?}", new_song);

//     song.display_song_info();
// }

// -- Builder pattern
// #[derive(Debug)]
// struct Computer {
//     cpu: String,
//     memory: u32,
//     storage: u32,
// }

// impl Computer {
//     fn new(cpu: String, memory: u32, storage: u32) -> Self {
//         Self {
//             cpu,
//             memory,
//             storage,
//         }
//     }

//     fn upgrade_cpu(&mut self, cpu: String) -> &mut Self {
//         self.cpu = cpu;
//         self
//     }

//     fn upgrade_memory(&mut self, memory: u32) -> &mut Self {
//         self.memory = memory;
//         self
//     }

//     fn upgrade_storage(&mut self, storage: u32) -> &mut Self {
//         self.storage = storage;
//         self
//     }
// }
// fn main() {
//     let mut computer = Computer::new(String::from("M3 chip"), 32, 520);

//     computer
//         .upgrade_cpu(String::from("M4 Max"))
//         .upgrade_memory(64)
//         .upgrade_storage(1000);

//     println!("{:?}", computer)
// }

// -- Tuple structs
// Hours and minutes
// struct ShortDuration(u32, u32);
// // years and months
// struct LongDuration(u32, u32);

// fn main() {
//     let work_shift = ShortDuration(8, 0);
//     println!("{} hours {} minutes", work_shift.0, work_shift.1);

//     let era = LongDuration(5, 3);
//     println!("{} year {} months", era.0, era.1);

//     go_to_work(work_shift);
// }

// fn go_to_work(length: ShortDuration) {
//     println!("Passing time {} hours {} minutes", length.0, length.1)
// }

// -- Unit like struct
// struct Empty;

// fn main() {
//     let my_empty_struct = Empty;
// }

/*
Define a Flight struct with the following fields:
  - an `origin` field (String)
  - a `destination` field (String)
  - a `price` field (f64)
  - a `passengers` field (u32)

Derive a Debug trait implementation for the Flight struct.

Define a `new` constructor function that returns a new
instance of a Flight.

Define a `change_destination` method that accepts a new
destination and overwrites the value of the `destination`
field.

Define a `increase_price` method that raises the value
of the `price` by 20% (multiply the `price` field by 1.20).
Make sure to save the new `price` field value.

Define a `itinerary` method that prints out both the
`origin` and `destination` fields in the following format
(origin -> destination).

Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.

Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Self {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, new_destination: String) {
        self.destination = new_destination
    }

    fn increase_price(&mut self) {
        self.price *= 1.2
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination)
    }
}

fn main() {
    let mut new_flight = Flight::new(
        String::from("Queenstown"),
        String::from("Auckland"),
        170.0,
        230,
    );
    println!("{:?}", new_flight);
    new_flight.change_destination(String::from("Sydney"));
    new_flight.increase_price();

    println!("{:?}", new_flight);
    new_flight.itinerary();

    let other_flight = Flight {
        origin: String::from("Tokyo"),
        destination: String::from("Auckland"),
        ..new_flight
    };

    println!("{:?}", other_flight);
    println!("{:?}", new_flight);
}
