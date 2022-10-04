// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox {
    length: f64,
    width: f64,
    height: f64,
    weight: f64,
    color: Color
}
impl ShippingBox {
    fn new() -> Self {
        Self {
            length: 10.0,
            width: 10.0,
            height: 10.0,
            weight: 10.0,
            color: Color::White
        }
    }
    fn print_characteristics(&self) {
        println!("Length: {:?}, Width: {:?}, Height: {:?}, Weight: {:?}, Color: {:?}",
                 self.length,
                 self.width,
                 self.height,
                 self.weight,
                 self.color.print());
    }
}
enum Color {
    Brown,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::White => println!("White"),
        }
    }
}

fn main() {
    let shipping_box = ShippingBox::new();
    shipping_box.print_characteristics();
}
