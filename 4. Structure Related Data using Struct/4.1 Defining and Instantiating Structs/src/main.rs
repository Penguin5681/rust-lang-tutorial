// Define a struct
struct Person {
    name: String,
    age: u32,
}

// Define a tuple struct
struct Point(i32, i32);

// Define a unit struct
struct EmptyStruct;

fn main() {
    // Accessing Struct Fields

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}", person1.name); // Accessing the 'name' field
    println!("Age: {}", person1.age);   // Accessing the 'age' field

    // Updating Struct Fields

    let mut person2 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    person2.age = 26; // Updating the 'age' field

    println!("Updated Age: {}", person2.age);

    // Tuple Structs

    let origin = Point(0, 0); // Creating an instance of a tuple struct
    println!("X: {}, Y: {}", origin.0, origin.1);

    // Unit Structs

    let empty = EmptyStruct; // Creating an instance of a unit struct
    // Unit structs don't have fields to access
}
