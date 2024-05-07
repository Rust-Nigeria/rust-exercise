// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use a struct to encapsulate the box characteristics
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn print_color(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
        }
    }
}

struct Box {
    dimensions: (f32, f32, f32),
    weight: i32,
    color: Color,
}

impl Box {
    fn new_box(&self, color: Color) -> Self {
        Self {
            dimensions: self.dimensions,
            weight: self.weight,
            color: color,
        }
    }
    fn print_box(&self) {
        let (x, y, z) = self.dimensions;
        self.color.print_color();
        println!("Dimensions: {:?} {:?} {:?}", x, y, z);
        println!("Weight: {:?}", self.weight);
    }
}

// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

fn main() {
    let green = Color::Green;
    let red = Color::Red;
    let blue = Color::Blue;
    red.print_color();

    let red_box = Box {
        dimensions: (10.4, 10.3, 10.2),
        weight: 10,
        color: red,
    };

    let small = Box::new_box(&red_box, green);
    small.print_box();
}
