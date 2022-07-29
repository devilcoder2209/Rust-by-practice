// Given

// FILL in the blanks
// use std::collections::HashMap;
// fn main() {
//     // type inference lets us omit an explicit type signature (which
//     // would be `HashMap<&str, u8>` in this example).
//     let mut player_stats = HashMap::new();

//     // insert a key only if it doesn't already exist
//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], __);

//     // insert a key using a function that provides a new value only if it
//     // doesn't already exist
//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], __);

//     // Ensures a value is in the entry by inserting the default if empty, and returns
//     // a mutable reference to the value in the entry.
//     let health = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, __);
//     *health -= 50;
//     assert_eq!(*health, __);

//     println!("Success!")
// }

// fn random_stat_buff() -> u8 {
//     // could actually return some random value here - let's just return
//     // some fixed value for now
//     42
// }

// My Solution
use std::collections::HashMap;
fn main() {
    let mut player_stats = HashMap::new();

    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100); // Add 100 because health doesn't already exist so we gave it a value of 100

    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100); // Health is still 100 because it already exists so we don't add the new value

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100); // Add &100 because health already exists and because .entry() returns a reference
    *health -= 50;
    assert_eq!(*health, 50); // Add 50 because 100 - 50 = 50

    println!("Success!")
}

fn random_stat_buff() -> u8 {
    // could actually return some random value here - let's just return
    // some fixed value for now
    42
}
