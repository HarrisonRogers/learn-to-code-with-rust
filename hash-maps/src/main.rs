use std::collections::{HashMap, HashSet};

// --- Hash map insert
// fn main() {
//     let mut menu: HashMap<String, f64> = HashMap::new();

//     menu.insert(String::from("Steak"), 28.99);
//     menu.insert(String::from("Salmon"), 28.99);
//     menu.insert(String::from("Cheese Burger"), 19.99);

//     println!("{:?}", menu);

//     let mut country_capitals: HashMap<&str, &str> = HashMap::new();

//     country_capitals.insert("France", "Paris");
//     country_capitals.insert("New Zealand", "Wellington");
//     println!("{:?}", country_capitals);
// }

// --- Removing HashMap
// fn main() {
//     let data = [("Bobby", 7), ("Harrison", 4), ("Allyx", 2)];

//     let mut years_at_company = HashMap::from(data);
//     println!("{:?}", years_at_company);

//     let harrison = years_at_company.remove("Harrison");
//     println!("{}", harrison.unwrap());
//     println!("{:?}", years_at_company);

//     let harrison = years_at_company.remove("Harrison");
//     println!("{:?}", harrison);
// }

// --- HashMap ownership and finding a value by key
// fn main() {
//     let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Flat White");
//     let milk = String::from("Skim Milk");
//     coffee_pairings.insert(&drink, &milk);
//     println!("{:?}", coffee_pairings);
//     println!("{} {}", milk, drink);

//     let value = coffee_pairings
//         .get("Flat White")
//         .copied()
//         .unwrap_or("Unknown Milk");
//     println!("{}", value)
// }

// --- Overwriting keys & entry methods
// fn main() {
//     let data = [
//         ("Jurassic Park", 1980),
//         ("Jaws", 1976),
//         ("Minecraft Movie", 2025),
//     ];
//     let mut movies_and_release_year = HashMap::from(data);
//     println!("{:?}", movies_and_release_year);

//     movies_and_release_year.insert("Jurassic Park", 1982);
//     println!("{:?}", movies_and_release_year);

//     movies_and_release_year.entry("Jaws").or_insert(1980);
//     movies_and_release_year.entry("Rocky").or_insert(1974);

//     println!("{:?}", movies_and_release_year)
// }

// --- HashSet
// fn main() {
//     let mut concrete_queue: HashSet<&str> = HashSet::new();
//     println!("{:?}", concrete_queue);

//     concrete_queue.insert("Harrison");
//     concrete_queue.insert("John");
//     println!("{:?}", concrete_queue);

//     concrete_queue.insert("Harrison");
//     println!("{:?}", concrete_queue);

//     println!("{}", concrete_queue.remove("John"));
//     println!("{}", concrete_queue.remove("Francis"));
//     println!("{:?}", concrete_queue);

//     println!("{}", concrete_queue.contains("Harrison"));
//     println!("{}", concrete_queue.contains("John"));

//     println!(
//         "{:?}",
//         concrete_queue
//             .get("Harrison")
//             .copied()
//             .unwrap_or("No entry with this name")
//     )
// }

// --- HashSet operations
// fn main() {
//     let mut concrete_queue: HashSet<&str> = HashSet::new(); // Boris, Jen
//     let mut movie_queue: HashSet<&str> = HashSet::new(); // Boris, Patrick

//     concrete_queue.insert("Boris");
//     concrete_queue.insert("Jen");

//     movie_queue.insert("Boris");
//     movie_queue.insert("Patrick");

//     println!("{:?}", concrete_queue.union(&movie_queue));
//     println!("{:?}", movie_queue.union(&concrete_queue));

//     println!("{:?}", concrete_queue.difference(&movie_queue));
//     println!("{:?}", movie_queue.difference(&concrete_queue));

//     println!("{:?}", concrete_queue.symmetric_difference(&movie_queue));
//     println!("{:?}", movie_queue.symmetric_difference(&concrete_queue));

//     println!("{:?}", concrete_queue.is_disjoint(&movie_queue));
//     println!("{:?}", movie_queue.is_disjoint(&concrete_queue));

//     println!("{:?}", concrete_queue.is_subset(&movie_queue));
//     println!("{:?}", movie_queue.is_subset(&concrete_queue));

//     let mut attendees = HashSet::new();
//     attendees.insert("Boris");
//     print!("{:?}", attendees.is_subset(&concrete_queue));
//     print!("{:?}", attendees.is_superset(&concrete_queue));
// }

// --- Project
fn main() {
    let sauces = [
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ];
    let mut sauces_to_meals = HashMap::from(sauces);

    sauces_to_meals.insert("Mustard", vec!["Hot Dog", "Burgers", "Pretzels"]);
    sauces_to_meals.remove("Mayonnaise");

    let mustard = sauces_to_meals.get("Mustard").expect("Sauces");
    println!("{:?}", mustard);

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{:#?}", sauces_to_meals)
}
