// We can also implement methods for enums

// Given

// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }

// // Implement TrafficLightColor with a method.
// impl TrafficLightColor {
    
// }

// fn main() {
//     let c = TrafficLightColor::Yellow;

//     assert_eq!(c.color(), "yellow");

//     println!("{:?}",c);
// }

// My Solution
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> String { // Make color function
        self.stringify() // Call stringify function to get the String version of the variant
    }
    
    fn stringify(&self) -> String {
        match self { // Use match statement to get the String
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}