fn main() {
    // Integers
    let positive_number: i32 = 42;   // A signed integer
    let natural_number: u64 = 100;   // An unsigned integer

    // Floating-Point Numbers
    let pi: f64 = 3.14159265359;    // A double-precision floating-point number

    // Booleans
    let is_rust_fun: bool = true;   // A boolean value

    // Characters
    let first_letter: char = 'A';   // A single Unicode character

    // Tuples
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true); // A tuple

    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // An array of integers

    // Structs
    struct Person {
        name: String,
        age: i32,
    }

    let alice = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Enums
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let current_light = TrafficLight::Red;

    // Printing values
    println!("Positive Number: {}", positive_number);
    println!("Natural Number: {}", natural_number);
    println!("Pi: {}", pi);
    println!("Is Rust Fun? {}", is_rust_fun);
    println!("First Letter: {}", first_letter);

    // Accessing Tuple Elements
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is Adult: {}", person.2);

    // Accessing Array Elements
    println!("Third Number: {}", numbers[2]);

    // Accessing Struct Fields
    println!("Name: {}", alice.name);
    println!("Age: {}", alice.age);

    // Matching Enums
    match current_light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go!"),
    }
}
