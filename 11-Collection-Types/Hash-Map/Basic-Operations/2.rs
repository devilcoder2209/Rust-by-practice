// Given

// use std::collections::HashMap;
// fn main() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }

//     // IMPLEMENT team_map2 in two ways
//     // tips: one of the approaches is to use `collect` method
//     let teams_map2...

//     assert_eq!(teams_map1, teams_map2);

//     println!("Success!")
// }

// My Solution
use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    // Way one: Create a mutable HashMap instance and iterate through teams, while pushing the values and keys to the HashMap Instance
    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // Way two: Use the .collect() method
    // First we need to turn teams into an iterateable
    let teams_map2 = teams.into_iter().collect(); // For some reason doesn't work on my local machine but is working in practice.rs
    
    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}