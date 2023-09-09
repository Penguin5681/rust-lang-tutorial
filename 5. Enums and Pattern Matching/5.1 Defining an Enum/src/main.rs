fn main() {
    let selected_color = Color::Blue;
    match selected_color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    let selected_shape = Shapes::Rectangle(15.5, 20.1);
    match selected_shape {
        Shapes::Circle(radius) => println!("The Radius of circle is: {} units", radius),
        Shapes::Rectangle(length, width) => println!("The dimensions of rectangle are: {}x{}", length, width),
    }
}

enum Color {
    Red, Green, Blue,
}

enum Shapes {
    Circle(f32),
    Rectangle(f32, f32),
}