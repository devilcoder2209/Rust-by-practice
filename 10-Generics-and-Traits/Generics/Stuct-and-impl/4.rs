// Given

// Modify this struct to make the code work
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     // DON'T modify this code.
//     let p = Point{x: 5, y : "hello".to_string()};

//     println!("Success!");
// }

// My Solution
struct Point<T, U> { // Add another type, i called it U
    x: T,
    y: U, // Use type U for y to make it independent of the type of x, so it can be a String in this case
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}