// Where vectors store values by an integer index, HashMaps store values by key. It is a hash map
// implemented with quadratic probing and SIMD lookup. By default, HashMap uses a hashing
// algorithm selected to procide resistance against HashDoS attacks.
// The default hashing algorithm is currently SiHash 1-3, though this is subject to change at any
// point in the future. While its performance is very competitive for medium sized keys, other hashing
// algorithms will outperform it for small keys such as integers as well as large keys as long 
// strings, though those algorithms will typically not protect against attacks suck as a HashDos.
// The hash tabel implementation is a Rust port of Google's SwissTable. The original C++ version of
// SwissTable can be found here, and this CppCon talk gives an overview of how the algorithm works. (Go to page to find the actaul links)

// Given

// FILL in the blanks and FIX the erros
// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69.0);
//     scores.insert("Katie", "58");

//     // get returns a Option<&V>
//     let score = scores.get("Sunface");
//     assert_eq!(score, Some(98));

//     if scores.contains_key("Daniel") {
//         // indexing return a value V
//         let score = scores["Daniel"];
//         assert_eq!(score, __);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), __);

//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score)
//     }
// }

// My Solution
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69); // Change to int 69 instead of float 69.0 
    scores.insert("Katie", 58); // Change str "58" to integer 58

    // get returns a Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98)); // Add & before int because .get() returns a reference to the value not the actual value

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, 95); // Add 95 as the value of Daniel is 95
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3); // Scores has 4 key-value pairs but we write 3 as the len because len of a hash map starts at 0

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}

