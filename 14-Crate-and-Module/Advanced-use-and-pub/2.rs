// If we are using multiple items degined in the same crate or module, then listing each item
// on its own line will take up too much vertical space.

// Given

// FILL in the blank in two ways
// DON'T add new code line
// use std::collections::__;

// fn main() {
//     let _c1:HashMap<&str, i32> = HashMap::new();
//     let mut c2 = BTreeMap::new();
//     c2.insert(1, "a");
//     let _c3: HashSet<i32> = HashSet::new();
// }

// My Solution

use std::collections::{HashMap, HashSet, BTreeMap}; // Do group import of HashMap, HashSet and BTreeMap

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}
