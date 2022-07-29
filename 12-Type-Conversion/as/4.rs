// Raw pointer can be converted to memory address (intefer) and vice versa.

// Given

// FILL in the blanks
// fn main() {
//     let mut values: [i32; 2] = [1, 2];
//     let p1: *mut i32 = values.as_mut_ptr();
//     let first_address: usize = p1 __; 
//     let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
//     let p2: *mut i32 = second_address __; // p2 points to the 2nd element in values
//     unsafe {
//         // add one to the second element
//         __
//     }
    
//     assert_eq!(values[1], 3);

//     println!("Success!")
// }

// My Solution
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; // We convert first_address to usize(unsigned 64 in my case)
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // We convert it to a mutable
    unsafe {
        // add one to the second element
        *p2 += 1;
    }
    
    assert_eq!(values[1], 3);

    println!("Success!")
}