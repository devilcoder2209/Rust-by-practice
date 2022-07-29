// Closures can capture variables by borrowing or moving. But they prefer to cpature by borrowing and
// only go lower when required:
// - by reference: &T
// - by mutable reference: &mut T
// - by value: T

// Given
/* Make it work with least changing */
// fn main() {
//     let color = String::from("green");

//     let print = move || println!("`color`: {}", color);

//     print();
//     print();

//     // `color` can be borrowed immutably again, because the closure only holds
//     // an immutable reference to `color`. 
//     let _reborrow = &color;

//     println!("{}",color);
// }

// My Solution
fn main() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color); // Remove move so the closure doesn't take ownership of the variable

    print();
    print();
    
    let _reborrow = &color;

    println!("{}",color);
}
