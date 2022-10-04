// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Cherry,
    Grape,
    Blueberry,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Cherry => println!("Cherry"),
        Flavor::Grape => println!("Grape"),
        Flavor::Blueberry => println!("Blueberry"),
    }
    println!("Oz: {:?}", drink.fluid_oz);
}

fn main() {
    let cherry = Drink {
        flavor: Flavor::Cherry,
        fluid_oz: 6.0
    };
    let grape = Drink {
        flavor: Flavor::Grape,
        fluid_oz: 10.0
    };
    let blueberry = Drink {
        flavor: Flavor::Blueberry,
        fluid_oz: 11.0
    };
    print_drink(cherry);
    print_drink(grape);
    print_drink(blueberry);
}
