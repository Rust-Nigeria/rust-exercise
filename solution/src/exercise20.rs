// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message


#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult{
    fn new (&self) -> Result<Self,&str>{
        if self.age >= 21 {
            Ok(Self{
                name: self.name.to_string(),
                age: self.age,
            })
        }else{
            Err("The person is under 21")
        }

    }
}
//   * The Err variant should contain a String (or &str) that explains why

fn main() {
    let adult = Adult{
        name: "John".to_string(), 
        age: 20
    };
    match adult.new() {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(err) => println!("Error: {}", err),
    }

  let child = Adult{
    name: "Jane".to_string(), 
    age: 21};

    match child.new(){
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(err) => println!("Error: {}", err),
    }
}
