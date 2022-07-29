// Given

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// 
// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup
// }
// 
// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};
// 
//     let p3 = p1.mixup(p2);
// 
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// 
//     println!("Success!");
// }

// My Solution
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<X, Y>(self, p: Point<X, Y>) -> Point<T, Y> { // Make generic function that takes any point with any data type and return value is a Point<datatype of first value of 1st point(the instance), datatype of second value of second point>
        Point {
            x: self.x, // First value of the point instance
            y: p.y, // Second value of p
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}