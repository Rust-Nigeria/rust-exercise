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


// * Use a struct for a persons age, name, and favorite color
struct Person{
    age: i32,
    name: String,
    favorite_color: String,
}
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


fn main() {
    let people = vec![
        Person{
            age: 10,
            name: "John".to_string(),
            favorite_color: "Red".to_string(),
        },
        Person{
            age: 11,
            name: "Jane".to_string(),
            favorite_color: "Green".to_string(),
        },
        Person{
            age: 9,
            name: "Doe".to_string(),
            favorite_color: "Blue".to_string(),
        },
    ];
    for person in people {
        if person.age <= 10 {
            println!("Name: {}", person.name);
            println!("Favorite Color: {}", person.favorite_color);
        }
    }
}
