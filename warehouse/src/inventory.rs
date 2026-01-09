// Submodule
pub mod products;

pub use products::{Item, ProductCategory};

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

fn talk_to_manager() {
    println!(
        "Hey {}, how is your coffee? What do you think of {:?}",
        MANAGER,
        ProductCategory::Ladder
    )
}

// Declaring from the bottom level
// pub fn talk_to_manager() {
//     println!("Hey {}, how is your coffee?", crate::inventory::MANAGER);
// }
