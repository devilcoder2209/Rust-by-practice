// The following four closures habe no difference in input and return types:
// - fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// - let add_one_v2 = |x: u32| -> u32 { x + 1 };
// - let add_one_v3 = |x|             { x + 1 };
// - let add_one_v4 = |x|               x + 1  ;

// Given

// fn main() {
//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));

//     /* Make it work, only changeg the following line */
//     let n = example_closure(5);
// }

// My Solution

fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    let n = example_closure(String::from("5")); // Give Strign 5 instead of int
}
