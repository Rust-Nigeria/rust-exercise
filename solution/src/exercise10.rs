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



// * Use an enum to create different flavors of drinks
 enum Drinks{
    Orange,
    Coke,
    Fanta,
    Lime,
 }
// * Use a struct to store drink flavor and fluid ounce information
struct DrinksFlavor {
    flavor: Drinks,
    fluid: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_drinks(flavor: DrinksFlavor){

    // * Use a match expression to print the drink flavor
    match flavor.flavor {
        Drinks::Orange => println!("orange"),
        Drinks::Coke => println!("coke"),
        Drinks::Fanta => println!("fanta"),
        Drinks::Lime => println!("lime"),
    }
    println!("{} ounces", flavor.fluid);

}




fn main() {
    let lime =  DrinksFlavor{
        flavor: Drinks::Lime,
        fluid: 1.0,
    };
    print_drinks(lime);
    let orange = DrinksFlavor{
        flavor: Drinks::Orange,
        fluid: 1.0,
    };
    print_drinks(orange);
    let coke = DrinksFlavor{
        flavor: Drinks::Coke,
        fluid: 1.0,
    };
    print_drinks(coke);
    let fanta = DrinksFlavor{
        flavor: Drinks::Fanta,
        fluid: 1.0,
    };
    print_drinks(fanta);
}
