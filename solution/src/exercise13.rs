// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

#[derive(Debug)]
struct Quantity{
    grocery: Box<str>,
    quantity: i32,
    id: i32,

}

fn display_quantity(grocery:&Quantity){
    println!("{:#?} {:?} {:?}",grocery.grocery,grocery.quantity,grocery.id);
}

fn main() {
   let grocery = Quantity {
    grocery :"garri".to_string().into(),
    quantity : 1,
    id : 4,

   };
   display_quantity(&grocery);
   display_quantity(&grocery);
   display_quantity(&grocery);
}
