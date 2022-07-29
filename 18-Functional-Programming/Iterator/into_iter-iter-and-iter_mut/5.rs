// Given

/* Fill in the blank */
// fn main() {
//     let mut names = vec!["Bob", "Frank", "Ferris"];

//     for name in names.__{
//         *name = match name {
//             &mut "Ferris" => "There is a rustacean among us!",
//             _ => "Hello",
//         }
//     }

//     println!("names: {:?}", names);
// }

// My Solution
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut(){ // Use .iter_mut() to turn names into an iteratable as we change values
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
