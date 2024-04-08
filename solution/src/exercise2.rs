// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result




// function to add two numbers together
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn main() {
    let result = add(1.0, 2.0);
    println!("The result is: {:?}", result);
}
