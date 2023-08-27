# User-Defined Functions in Rust

In Rust, user-defined functions are a way to organize and reuse code. They allow you to create your custom blocks of code that perform specific tasks. Think of functions as building blocks for your Rust programs.

## Creating a Function

To create a function, you need to follow these basic steps:

### 1. Function Declaration

- Start by declaring your function using the `fn` keyword.
- Give your function a meaningful name that describes what it does.

Example:
```rust
fn greet() {
    // Function code goes here
}
```

### 2. Parameters (Optional)

- If your function needs input data, you can define parameters inside parentheses.
- Parameters act as placeholders for values you want to use within the function.

Example:
```rust
fn greet(name: &str) {
    // Function code that uses the 'name' parameter
}
```

### 3. Function Body

- The function body contains the code that gets executed when the function is called.
- It can perform calculations, manipulate data, or perform any specific task.

Example:
```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

## Calling a Function

Once you've defined a function, you can use it by calling it from other parts of your code.

### Function Call

- To call a function, simply use its name followed by parentheses.
- If your function has parameters, provide the required values inside the parentheses.

Example:
```rust
fn main() {
    greet("Alice");
}
```

## Return Values

Functions can also return values, allowing you to get results from them.

### Return Type

- When defining a function, specify the return type using an arrow (`->`) followed by the type.

Example:
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y  // This function returns the sum of 'x' and 'y'
}
```

### Using Return Values

- To use the value returned by a function, assign it to a variable or use it in an expression.

Example:
```rust
fn main() {
    let result = add(5, 3);  // Call the 'add' function and store the result
    println!("Result: {}", result);
}
```
