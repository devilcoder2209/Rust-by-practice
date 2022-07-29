// The simplest error handling mechanism is to use panic. It just prints an error message and starts 
// unwinding the stack, finally exit the current thread:
// - if panic occured in main thread, then the program will be exited.
// - if in spawned thread, then the thread will be terminated, but the program won't

// Given

// FILL the blanks
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // IMPLEMENT the below code
//         __
//      }

//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     drink(__);

//     println!("Exercise Failed if printing out this line!");
// }

// My Solution
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        
        panic!("BOOOOOO"); // Panic so we don't continue
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade"); // Provide lemonade as the parameter so the program panics

    println!("Exercise Failed if printing out this line!");
}
