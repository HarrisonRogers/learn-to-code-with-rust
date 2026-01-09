// --- Intro
// fn main() {
//     let pizza_diameters = vec![8, 10, 12, 14];
//     // let pizza_diameters = Vec::<i32>::new();
//     println!("{:?}", pizza_diameters);

//     let pastas = vec!["pene", "fetuchini"];
//     println!("{:?}", pastas);
// }

// -- Adding and removing values
// fn main() {
//     let mut pizza_diameters = vec![2, 4, 5, 6];
//     pizza_diameters.push(9);
//     pizza_diameters.push(2);

//     pizza_diameters.insert(2, 12);

//     let last_pizza_diameter = pizza_diameters.pop();
//     println!("{:?}", last_pizza_diameter);

//     pizza_diameters.remove(3);

//     println!("{:?}", pizza_diameters);
// }

// --- Rendering vector elements
// fn main() {
//     let pizza_diameters = vec![8, 10, 12, 14];

//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Sausage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];

//     let pizza_slice = &pizza_toppings[..2];
//     println!("{:?}", pizza_slice)
// }

// --- Get method
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Sausage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];

//     let topping = pizza_toppings.get(0);

//     match topping {
//         Some(value) => println!("The topping is {}", value),
//         None => println!("There is no value"),
//     }
// }

// --- Ownership with vectors
// fn main() {
//     let bacon = String::from("Bacon");
//     let chicken = String::from("Chicken");
//     let pepperoni = String::from("Pepperoni");
//     let pizza_toppings = vec![bacon, chicken, pepperoni];
//     let mut delicious_toppings = pizza_toppings;

//     let topping_reference = &delicious_toppings[1];
//     println!("The topping is {}", topping_reference);

//     delicious_toppings.push(String::from("Olives"));
// }

// --- Writing Vector elements
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Sausage");
//     let mut pizza_toppings = vec![pepperoni, mushroom, sausage];

//     pizza_toppings[1] = String::from("Bacon");

//     println!("{:#?}", pizza_toppings);

//     let target_topping = &mut pizza_toppings[2];
//     target_topping.push_str(" and Meatballs");

//     println!("{:#?}", pizza_toppings)
// }

// --- Vector capacity behind the scenes
// fn main() {
//     let mut seasons: Vec<&str> = Vec::with_capacity(4);
//     println!(
//         "Length: {}, Capacity: {}",
//         seasons.len(),
//         seasons.capacity()
//     );

//     seasons.push("Summer");
//     seasons.push("Autumn");
//     seasons.push("Winter");
//     seasons.push("Spring");
//     println!(
//         "Length: {}, Capacity: {}",
//         seasons.len(),
//         seasons.capacity()
//     );

//     seasons.push("Summer");

//     println!(
//         "Length: {}, Capacity: {}",
//         seasons.len(),
//         seasons.capacity()
//     );
// }

// --- Project
#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: Vec::new(),
        }
    }

    fn create_file(&mut self, name: String) {
        let new_file = File { name };
        self.contents.push(new_file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}
fn main() {
    let mut folder = Folder::new(String::from("Desktop"));
    folder.create_file(String::from("code-stuff"));
    folder.create_file(String::from("photos"));
    println!("{:?}", folder);

    folder.delete_file(1);
    println!("{:?}", folder);

    match folder.get_file(0) {
        Some(file) => println!("This is the file {:?}", file.name),
        None => println!("There is no file"),
    }
}
