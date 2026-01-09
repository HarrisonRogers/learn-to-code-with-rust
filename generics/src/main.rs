// #[derive(Debug)]
// struct DeliSandwich {}

// fn main() {
//     let bool_identity = identity(true);
//     let i32_identity = identity::<i8>(4); // Can label this type as any so i32 or i64 or i16 or i8 but also don't need too
//     let string_identity = identity(String::from("Me"));

//     println!("Identity is a {}", bool_identity);
//     println!("Identity is a {}", i32_identity);
//     println!("Identity is a {}", string_identity);
//     println!("Identity is a {:?}", identity(DeliSandwich {}));
// }

// fn identity<T>(value: T) -> T {
//     value
// }

// --- Multiple generics
// fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
//     (first, second)
// }

// fn main() {
//     println!("{:?}", make_tuple(String::from("Hello"), 8));
//     println!("{:?}", make_tuple(3, 5.99));
//     println!("{:?}", make_tuple(true, false));
// }

// --- Generics in structs && impl blocks
// #[derive(Debug)]
// struct TreasureChest<T> {
//     captain: String,
//     treasure: T,
// }

// impl TreasureChest<String> {
//     fn clean_treasure(&mut self) {
//         self.treasure = self.treasure.trim().to_string();
//     }
// }

// impl TreasureChest<[&str; 3]> {
//     fn amount_of_treasure(&self) -> usize {
//         self.treasure.len()
//     }
// }

// fn main() {
//     let gold_chest = TreasureChest {
//         captain: String::from("Blackbeard"),
//         treasure: "Gold",
//     };
//     println!("{:?}", gold_chest);

//     let mut silver_chest = TreasureChest {
//         captain: String::from("Jack Sparrow"),
//         treasure: String::from("    Silver    "),
//     };
//     silver_chest.clean_treasure();
//     println!("{:?}", silver_chest);

//     let special_chest = TreasureChest {
//         captain: String::from("Harrison Rogers"),
//         treasure: ["Gold", "Silver", "Diamonds"],
//     };
//     println!("Amount of treasure {}", special_chest.amount_of_treasure());
//     println!("{:?}", special_chest);
// }

// --- Impl blocks 2
// #[derive(Debug)]
// struct TreasureChest<T> {
//     captain: String,
//     treasure: T,
// }

// impl TreasureChest<String> {
//     fn clean_treasure(&mut self) {
//         self.treasure = self.treasure.trim().to_string();
//     }
// }

// impl TreasureChest<[&str; 3]> {
//     fn amount_of_treasure(&self) -> usize {
//         self.treasure.len()
//     }
// }

// impl<T> TreasureChest<T> {
//     fn capital_captain(&self) -> String {
//         self.captain.to_uppercase()
//     }
// }

// fn main() {
//     let gold_chest = TreasureChest {
//         captain: String::from("Blackbeard"),
//         treasure: "Gold",
//     };
//     println!("{:?}", gold_chest.capital_captain());

//     let mut silver_chest = TreasureChest {
//         captain: String::from("Jack Sparrow"),
//         treasure: String::from("    Silver    "),
//     };
//     silver_chest.clean_treasure();
//     println!("{:?}", silver_chest.capital_captain());

//     let special_chest = TreasureChest {
//         captain: String::from("Harrison Rogers"),
//         treasure: ["Gold", "Silver", "Diamonds"],
//     };
//     println!("Amount of treasure {}", special_chest.amount_of_treasure());
//     println!("{:?}", special_chest.capital_captain());
// }

// --- Generics in enums
// enum Cheesesteak<T> {
//     Plain,
//     Topping(T),
// }

// fn main() {
//     let mushroom = Cheesesteak::Topping("mushroom");
//     let onions = Cheesesteak::Topping(String::from("onions"));
//     let topping = String::from("bacon");
//     let bacon = Cheesesteak::Topping(&topping);

//     let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
//     plain = Cheesesteak::Topping(String::from("potato"));
// }

/*
Let's model a real-time chat system where users can
share audio and video files.

Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation.

Define a ChatMessage struct with two fields: `content`
and `time`. The struct should define one generic type, T,
which will be the type of the `content` field.
The `time` field should always be a String.
Derive a Debug implementation.

Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile".

Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct.

In `main`, create a ChatMessage with `content` set to a
string slice.

Create another ChatMessage with `content` set to a String.

Create another ChatMessage with `content' set to a
DigitalContent variant.

Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum.

Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/
#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message = ChatMessage {
        content: "Youtube",
        time: String::from("3pm"),
    };
    let message_2 = ChatMessage {
        content: String::from("Spotify"),
        time: String::from("3pm"),
    };
    let message_3 = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("8am"),
    };

    message_3.consume_entertainment();
    println!(
        "{}, {}, {}",
        message.retrieve_time(),
        message_2.retrieve_time(),
        message_3.retrieve_time()
    );
}
