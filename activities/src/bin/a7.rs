// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Blue,
    Green,
    Purple
}
fn main() {
    let color: Color = Color::Red;

    match color {
        Color::Red => println!("You picked red!"),
        Color:: Blue => println!("You picked blue!"),
        Color::Green => println!("You picked green!"),
        Color::Purple => println!("You picked purple!")
    }
}
