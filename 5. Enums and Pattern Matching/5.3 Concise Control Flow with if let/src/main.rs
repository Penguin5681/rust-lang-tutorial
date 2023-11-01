fn main() {
    let some_value: Option<i32> = Some(42);

    if let Some(value) = some_value {
        println!("Found a value: {}", value);
    } else {
        println!("No value found.");
    }
}

// You'll find some other examples in the readme.md file
