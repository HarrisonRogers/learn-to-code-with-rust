// fn main() {
//     let apples = 3;
//     let mangoes = apples * 5 + 3;
//     println!("I have {apples} apples");
//     println!("I have {} mangoes", mangoes);
// }

#![allow(unused_variables)]
type Beverage = String;

const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "summer";
    let mut points_scored: i32 = 28;
    points_scored = 35;

    let event_time: &str = "6:00";
    let event_time: i32 = 6;
    println!(
        "Season: {season}, Points Scored: {points_scored}, Touchdown Points: {TOUCHDOWN_POINTS}, Event Time: {event_time}"
    );
    let favorite_beverage: Beverage = String::from("Coke Zero");
    println!("My favorite beverage is {favorite_beverage}");
}
