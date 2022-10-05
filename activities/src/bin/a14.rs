// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String
}
fn print_name(person: &Person) {
    println!("{:?}", person.name);
}
fn print_favorite_color(person: &Person) {
    println!("{:?}", person.favorite_color);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Rhett"),
            age: 24,
            favorite_color: String::from("Red"),
        },
        Person {
            name: String::from("Bailee"),
            age: 22,
            favorite_color: String::from("Purple"),
        },
        Person {
            name: String::from("Bob"),
            age: 10,
            favorite_color: String::from("Green"),
        },
        Person {
            name: String::from("Alice"),
            age: 8,
            favorite_color: String::from("Yellow"),
        }
    ];
    for person in people {
        if person.age <= 10 {
            print_name(&person);
            print_favorite_color(&person);
        }
    }
}
