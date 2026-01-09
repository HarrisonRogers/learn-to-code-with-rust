// --- Declaring enums
// #[derive(Debug)]
// enum CardSuit {
//     Hearts,
//     Diamonds,
//     Spades,
//     Clubs,
// }

// struct Card {
//     rank: String,
//     suit: CardSuit,
// }

// fn main() {
//     let first_card = CardSuit::Hearts;
//     let mut second_card = CardSuit::Diamonds;
//     second_card = CardSuit::Spades;
//     println!("{:?}", first_card);

//     let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
//     let card_suits = (CardSuit::Hearts, CardSuit::Clubs);
// }

// --- Enum with associated values
// #[derive(Debug)]
// enum PaymentMethodType {
//     CreditCard(String),
//     DebitCard(String),
//     ApplePay(String, String),
//     Cash(u32),
// }

// fn main() {
//     // let visa = PaymentMethodType::CreditCard(String::from("4865-0322-3848-1929-00"));
//     // let cash = PaymentMethodType::Cash(180);

//     // println!("{:?}", visa);
//     // println!("{:?}", cash);

//     let mut my_payment_method = PaymentMethodType::CreditCard(String::from("459-3920-10203"));

//     my_payment_method =
//         PaymentMethodType::ApplePay(String::from("Harrison"), String::from("038209430"));

//     println!("{:?}", my_payment_method);
// }

// --- Using structs for enum variants
// #[derive(Debug)]
// struct ApplePayFields {
//     username: String,
//     id: u32,
// }

// #[derive(Debug)]
// enum PaymentMethodType {
//     CreditCard(String),
//     Cash(u32),
//     // ApplePay(ApplePayFields),
//     ApplePay { username: String, id: u32 },
// }

// fn main() {
//     let apple_pay_credentials = ApplePayFields {
//         username: String::from("hrogerz"),
//         id: 83240,
//     };

//     // let apple_pay = PaymentMethodType::ApplePay(apple_pay_credentials);
//     let apple_pay = PaymentMethodType::ApplePay {
//         username: String::from("hrogerz"),
//         id: 83240,
//     };
//     println!("{:?}", apple_pay)
// }

// -- Nesting Enums in Enums
// #[derive(Debug)]
// enum Beans {
//     Pinto,
//     Black,
// }

// #[derive(Debug)]
// enum Meat {
//     Chicken,
//     Beef,
// }

// #[derive(Debug)]
// enum RestaurantItem {
//     Burrito { meat: Meat, beans: Beans },
//     Bowl(Meat),
//     VeganPlate,
// }
// fn main() {
//     let lunch = RestaurantItem::Burrito {
//         meat: Meat::Beef,
//         beans: Beans::Black,
//     };
//     let dinner = RestaurantItem::Bowl(Meat::Chicken);
//     let terrible_meal = RestaurantItem::VeganPlate;

//     println!("Lunch was {:?} and dinner was {:?}", lunch, dinner);
//     println!("Nobody ate {:?}", terrible_meal);
// }

// Match with Enums
// #[derive(Debug)]
// enum OperatingSystem {
//     Windows,
//     MacOS,
//     Linux,
// }
// fn main() {
//     let my_computer = OperatingSystem::MacOS;
//     let age = years_since_release(my_computer);
//     println!("My computer operation system is {} years old", age);

//     let dads_computer = OperatingSystem::Windows;
//     let age = years_since_release(dads_computer);
//     println!("My dads computer is {} years old", age);
// }

// fn years_since_release(os: OperatingSystem) -> u32 {
//     match os {
//         OperatingSystem::Windows => {
//             println!("Pretty old");
//             39
//         }
//         OperatingSystem::Linux => 34,
//         OperatingSystem::MacOS => 24,
//     }
// }

// -- Match keyword && Methods
// #[derive(Debug)]
// enum LaundryCycle {
//     Cold,
//     Hot { temperature: u32 },
//     Delicate(String),
// }

// impl LaundryCycle {
//     fn wash_laundry(&self) {
//         match self {
//             Self::Cold => {
//                 println!("Running the laundry with cold temperature")
//             }
//             Self::Hot { temperature } => {
//                 println!("Running the laundry with a temperature of {}", temperature)
//             }
//             Self::Delicate(fabric_type) => {
//                 println!(
//                     "Running the laundry with a delicate cycle for {}",
//                     fabric_type
//                 )
//             }
//         }
//     }
// }

// fn main() {
//     LaundryCycle::Cold.wash_laundry();
//     let hot_cycle = LaundryCycle::Hot { temperature: 100 };
//     let delicate_cycle = LaundryCycle::Delicate(String::from("Cotton"));

//     hot_cycle.wash_laundry();
//     delicate_cycle.wash_laundry();
// }

// // fn wash_laundry(cycle: LaundryCycle) {
// //     match cycle {
// //         LaundryCycle::Cold => {
// //             println!("Running the laundry with cold temperature")
// //         }
// //         LaundryCycle::Hot { temperature } => {
// //             println!("Running the laundry with a temperature of {}", temperature)
// //         }
// //         LaundryCycle::Delicate(fabric_type) => {
// //             println!(
// //                 "Running the laundry with a delicate cycle for {}",
// //                 fabric_type
// //             )
// //         }
// //     }
// // }

// -- Catching multiple values
// #[derive(Debug)]
// enum OnlineOrderStatus {
//     Ordered,
//     Packed,
//     Shipped,
//     Delivered,
// }

// impl OnlineOrderStatus {
//     fn check(&self) {
//         match self {
//             // OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
//             //     println!("Your item is being prepped for shipment")
//             // }
//             OnlineOrderStatus::Delivered => {
//                 println!("Your item has been delivered");
//             }
//             other_status => {
//                 println!("Your order is {:?}", other_status);
//             }
//         }
//     }
// }

// fn main() {
//     OnlineOrderStatus::Packed.check();
// }

// -- Match with exact value
// enum Milk {
//     Lowfat(i32),
//     Whole,
// }

// impl Milk {
//     fn drink(self) {
//         match self {
//             Milk::Lowfat(2) => println!("Delicious 2% milk"),
//             Milk::Lowfat(percent) => println!("You got the lowfat {}% milk", percent),
//             Milk::Whole => println!("Whole milk"),
//         }
//     }
// }

// fn main() {
//     Milk::Lowfat(45).drink();
//     Milk::Whole.drink();
// }

// -- if let statement
// enum Milk {
//     LowFat(i32),
//     Whole,
//     NonDiary { kind: String },
// }

// fn main() {
//     // let my_beverage = Milk::LowFat(4);

//     // if let Milk::LowFat(percent) = my_beverage {
//     //     println!("You have {}% low fat milk", percent)
//     // }

//     let my_beverage = Milk::NonDiary {
//         kind: String::from("Almond"),
//     };

//     if let Milk::NonDiary { kind } = my_beverage {
//         println!("Your kind of milk is {}", kind)
//     }
// }

// -- let else statment
// enum Milk {
//     LowFat(i32),
//     Whole,
//     NonDiary { kind: String },
// }

// fn main() {
//     let my_beverage = Milk::LowFat(4);

//     let Milk::LowFat(percent) = my_beverage else {
//         println!("Not a low fat milk");
//         return;
//     };

//     println!("{percent}% milk is available here");
// }

/*
Define a Tier enum with three variants: Gold, Silver,
Platinum. Derive a Debug implementation for the Tier enum.

Define a Subscription enum with three variants: Free,
Basic, and Premium. Derive a Debug implementation for the
Subscription enum.

The Free variant should have no associated data.

The Basic variant should be a tuple variant with two pieces
of data. The first one should be a f64 (the price per month)
and the second one should be a u32 (the number of months).

The Premium variant should be a struct variant with a 'tier'
field. The tier field should be a Tier enum value.

Define a 'summarize' method on the Subscription enum.

If the Subscription is Free, output the text "You have
limited access to the site".

If the Subscription is Basic, output the text "You have
limited access to the site's premium features for {price}
for {months} months", where {price} amd {months} come from
the tuple variant's associated data.

If the Subscription is Premium, output the text "You have
full access to the site's premium features. Your tier is
{tier:?}"", where {tier} comes from the struct variant's
associated 'tier' field.

In the `main` function, create 3 instances of Subscription,
each one with a different variant. Invoke the `summarize`
method on each instance.
*/

#[derive(Debug)]
enum Tier {
    Silver,
    Gold,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => println!("You have limited access"),
            Subscription::Basic(price, months) => println!(
                "You have limited access to the site's premium features for {} for {} month",
                price, months
            ),
            Subscription::Premium { tier } => println!(
                "You have full access to the site's premium features. Your tier is {:?}",
                tier
            ),
        }
    }
}

fn main() {
    Subscription::Free.summarize();
    Subscription::Basic(10.99, 1).summarize();
    Subscription::Premium { tier: Tier::Gold }.summarize();
}
