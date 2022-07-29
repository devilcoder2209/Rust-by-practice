// When taking a closure as an input parameter, the closure's complete type must be annotated using
// one of the following traits:
// - Fn: The closure uses the captured value by reference (&T)
// - FnMut: The closure uses teh captured value by mutable reference (&mut T)
// - FnOnce: The closure uses the captured value by value (T)

// Given

/* Make it work by change the trait bound, in two ways*/
// fn fn_once<F>(func: F)
// where
//     F: FnOnce(usize) -> bool,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x = vec![1, 2, 3];
//     fn_once(|z|{z == x.len()})
// }

// My Solution

fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool, // Change to Fn so we don't move it
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}