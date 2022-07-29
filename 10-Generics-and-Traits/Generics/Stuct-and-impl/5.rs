// Given

// Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val {
//     val: f64,
// }

// impl Val {
//     fn value(&self) -> &f64 {
//         &self.val
//     }
// }


// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }

// My Solution
struct Val<T> { // Make Val generic
    val: T, // Use generic type T instead of a concrete type
}

impl<T> Val<T> { // Make implement block generic
    fn value(&self) -> &T { // The function returns generic type T now, not a concrete type, so change it to &T
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}