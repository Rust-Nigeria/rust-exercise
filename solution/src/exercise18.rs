// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Student {
    name: String,
    locker: Option<i32>,
}
impl Student {
    fn new(&self) -> Self {
        Self{
            name: self.name.to_string(),
            locker: self.locker,
        }
    }
    fn print_locker(&self) {
        match self.locker {
            Some(locker) => println!("{} has locker number {}", self.name, locker),
            None => println!("{} does not have a locker", self.name),
        }
    }
}

fn main() {
    let s = Student{
        name: String::from("John"),
        locker: Some(32),
    };
   


   let student = Student::new(&s);
    student.print_locker();
}
