// Yeah, Debug is simple and easy to use. But sometimes we want to customize the output appearance
// of our type. This is where display really shines.
// Unlike Debug, there is no way to derive the implementation of the Display trait, we have to
// manually implement it.
// Another thing to note: the place holder for Display is {} not {:?}.

// Given
// /* Make it work */
// use std::fmt;

// struct Point2D {
//     x: f64,
//     y: f64,
// }

// impl fmt::Display for Point2D {
//     /* Implement.. */
// }

// impl fmt::Debug for Point2D {
//     /* Implement.. */
// }

// fn main() {
//     let point = Point2D { x: 3.3, y: 7.2 };
//     assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
//     assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
//     println!("Success!")
// }

// My Solution
use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
    println!("Success!")
}
