// Methods are similar to functions: Declare with fn, have parameters and a return value.
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), 
// and their first parameter is always self, which represents the instance of the stuct the mehtod is 
// being called on.
//
// Given
//
/*struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Complete the area method which return the area of a Rectangle.
    fn area
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}*/

// My Solution
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Make the function return the area
    fn area(&self) -> u32 {
        self.width * self.height // We return the area by multipying the width and height of the instance
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}