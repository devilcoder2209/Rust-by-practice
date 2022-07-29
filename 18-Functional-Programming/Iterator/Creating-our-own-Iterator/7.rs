// We can not only create iterators from collection types, but also by
// implementing the Iterator trait on our own types.

// Given

// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }

// // Implement `Iterator` for `Fibonacci`.
// // The `Iterator` trait only requires a method to be defined for the `next` element.
// impl Iterator for Fibonacci {
//     // We can refer to this type using Self::Item
//     type Item = u32;
    
//     /* Implement next method */
//     fn next(&mut self)
// }

// // Returns a Fibonacci sequence generator
// fn fibonacci() -> Fibonacci {
//     Fibonacci { curr: 0, next: 1 }
// }

// fn main() {
//     let mut fib = fibonacci();
//     assert_eq!(fib.next(), Some(1));
//     assert_eq!(fib.next(), Some(1));
//     assert_eq!(fib.next(), Some(2));
//     assert_eq!(fib.next(), Some(3));
//     assert_eq!(fib.next(), Some(5));
// }

// My Solution
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next; // Set current to next
        self.next = new_next; // Set the next value to self.curr + self.next

        Some(self.curr) // Return the current value
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}
