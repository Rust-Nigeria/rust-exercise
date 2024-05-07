// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

// the below code fragment can be found in:

fn main() {
    let v = vec![10, 20, 30, 40];
    for i in &v {
        match i {
            30 => println!("thirty"),
            _ => println!("{}", i),
        }
    }

    println!("The number of elements in the vector is {}", v.len());

    let numbers = vec![10, 20, 30, 40];
    for num in &numbers {
        if *num == 30 {
            println!("thirty");
        } else {
            println!("{}", num);
        }
    }
    println!("{:?}", numbers.len());
}
