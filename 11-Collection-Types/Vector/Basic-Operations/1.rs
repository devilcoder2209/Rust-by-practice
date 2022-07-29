// Vectors are re-sizable arrays. Like slices, their size is not known
// at complile time, but they can grow or shrink at any time.

// Given

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     let v = Vec::from(arr);
//     is_vec(v);

//     let v = vec![1, 2, 3];
//     is_vec(v);

//     // vec!(..) and vec![..] are same macros, so
//     let v = vec!(1, 2, 3);
//     is_vec(v);
    
//     // in code below, v is Vec<[u8; 3]> , not Vec<u8>
//     // USE Vec::new and `for` to rewrite the below code 
//     let v1 = vec!(arr);
//     is_vec(v1);
 
//     assert_eq!(v, v1);

//     println!("Success!")
// }

// fn is_vec(v: Vec<u8>) {}

// My Solution
fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v = Vec::from(arr);
    is_vec(&v); // Add & to give reference

    let v = vec![1, 2, 3];
    is_vec(&v);

    let v = vec!(1, 2, 3);
    is_vec(&v);
    
    // in code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code 
    let mut v1 = Vec::new(); // Create new empty vector

    for e in arr.iter() { // Iterate through arr and push the elements to the Vector
        v1.push(*e) // Add dereferenced value to v1
    }

    is_vec(&v1);
 
    assert_eq!(v, v1);

    println!("Success!")
}

fn is_vec(v: &Vec<u8>) {} // Change type to use reference so no borrowing occurs