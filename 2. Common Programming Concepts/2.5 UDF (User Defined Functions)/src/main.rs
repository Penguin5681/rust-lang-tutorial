fn main() {
    println!("We are Inside the main function");
    some_function();

    println!();

    add(12, 4);

    println!();

    println!("{}", multiply(15, 2));
}

// Function without parameters and return type
fn some_function() {
    println!("We are under a user defined function");
}

// Functions with parameters
fn add(a :i32, b :i32) {
    println!("{}", a + b);
}

// Functions with parameters and return type
fn multiply(a :i32, b :i32) -> i32 {
    a * b   // returning an expression requires no semicolon
}