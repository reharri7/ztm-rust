// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Permimeter {
    fn print_permimeter(&self) -> f64;
}

struct Square {
    length: i32,
}
impl Permimeter for Square {
    fn print_permimeter(&self) -> f64 {
        (self.length * 4) as f64
    }
}

// implement perimeter for triangle
struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}
impl Permimeter for Triangle {
    fn print_permimeter(&self) -> f64 {
        (self.a + self.b + self.c) as f64
    }
}

fn main() {
    let square = Square { length: 5 };
    let triangle = Triangle { a: 3, b: 4, c: 5 };

    println!("{}", square.print_permimeter());
    println!("{}", triangle.print_permimeter());
}
