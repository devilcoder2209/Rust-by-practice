// Given

/* Fill in the blanks */
// use std::collections::HashMap;
// fn main() {
//     let names = ["sunface", "sunfei"];
//     let ages = [18, 18];
//     let folks: HashMap<_, _> = names.into_iter().__.collect();

//     println!("{:?}",folks);
// }

// My Solution
use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().map(|x| (x, ages[names.iter().position(|&y| y == *x).unwrap()])).collect(); // Use map to make each element have a value of names and a value of age, for that we get the index of the current element and then we use that index to get the corresponding age

    println!("{:?}",folks);
}
