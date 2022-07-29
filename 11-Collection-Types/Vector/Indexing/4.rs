// Given

// FIX the error and IMPLEMENT the code
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v[i])
//     }

//     for i in 0..5 {
//        // IMPLEMENT the code here...
//     }
    
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!")
// }

// My Solution
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        match v.get(i) { // We use .get() to get an Option enum and then we print different things if the element at index i exists or not
            Some(t) => println!("{:?}", t),
            None => println!("No element at index {}", i),
        }
    }

    for i in 0..5 {
        let value = v.get(i); // get the value at the index i

        match value { // Use match expression to do different things based on if the element exists or not
            Some(t) => {
                let new_val = t + 1; // The value is now value + 1 if the element exists
                v[i] = new_val;
            },
            None => {
                let prev_val = v[i - 1]; // If the element doesn't exist we get the previous element and add one to it and push it to the vector
                v.push(prev_val + 1); // As we know that there must be a previous element we can simply use indexing instead of using .get() and writing another match expression
             
            }
        }
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}