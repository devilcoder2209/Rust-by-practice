// Any type that implements the Eq and Hash traits can be a key in HashMap. This includes:
// -bool
// -int and uint
// -String and &str
// Note that f32 and f64 do not implement Hash, likely because floating-point percision errors
// would make using them as hashmap keys horribly error-prone.
// All collection classes implement Eq and Hash if their contained type also respectively implements
// Eq and Hash. For example, Vec<T> will implement Hash if T implements Hash.

// Given

// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits
// use std::collections::HashMap;

// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     /// Creates a new Viking.
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn main() {
//     // Use a HashMap to store the vikings' health points.
//     let vikings = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);

//     // Use derived implementation to print the status of the vikings.
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// }

// My Solution
use std::collections::HashMap;
// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits

#[derive(Debug)] // Add debug trait so Viking can be printed
#[derive(Hash)] // Add hash trait 
#[derive(Eq, PartialEq)] // PartialEq and Eq traits
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
