// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal


// function to display your first name
fn first_name() -> String {
    "mart".to_string()
}


// function to display your last name
fn last_name() -> String {
    "ola".to_string()
}



fn main() {

    println!("my name is {} {}", first_name(), last_name());


}
