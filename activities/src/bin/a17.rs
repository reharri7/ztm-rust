// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let string_to_print = "CamelCaseString".to_owned();
    let lowercase_string = string_to_print.to_lowercase();
    let uppercase_string = string_to_print.to_uppercase();
    println!("{:?}", string_to_print);
    println!("{:?}", lowercase_string);
    println!("{:?}", uppercase_string);
}
