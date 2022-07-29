// Given

// fn main() {
//     let mut s = String::new();

//     let update_string = |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}",s);
// }

// /* Fill in the blank */
// fn exec<'a, F: __>(mut f: F)  {
//     f("hello")
// }

// My Solution

fn main() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

/* Fill in the blank */
fn exec<'a, F: FnMut(&'a str)>(mut f: F)  { // FnMut because we mutate s and we need to specify a lifetime
    f("hello")
}
