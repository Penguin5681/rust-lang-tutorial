# Control Flow Statements in Rust

Control flow statements are the building blocks of a Rust program that determine the order in which instructions are executed. They allow you to make decisions, repeat actions, and create flexible program logic.

## Conditional Statements

### 1. **if-else Statements**

- Use `if` statements to make decisions based on conditions.
- If the condition is true, the code within the `if` block is executed; otherwise, the code within the `else` block is executed.

Example:
```rust
let age = 20;

if age >= 18 {
    println!("You are an adult.");
} else {
    println!("You are a minor.");
}
```

### 2. **match Statements**

- `match` statements are used for pattern matching.
- They allow you to compare a value against a set of patterns and execute code based on the match.

Example:
```rust
let choice = "apple";

match choice {
    "apple" => println!("You chose an apple."),
    "banana" => println!("You chose a banana."),
    _ => println!("Unknown choice."),
}
```

## Loops

### 1. **loop Statement**

- The `loop` statement creates an infinite loop.
- You can use it when you want to repeat an action until a specific condition is met.

Example:
```rust
let mut counter = 0;

loop {
    println!("Count: {}", counter);
    counter += 1;

    if counter >= 5 {
        break; // Exit the loop when the condition is met
    }
}
```

### 2. **while Statement**

- `while` loops repeatedly execute a block of code as long as a specified condition is true.

Example:
```rust
let mut countdown = 5;

while countdown > 0 {
    println!("Countdown: {}", countdown);
    countdown -= 1;
}
```

### 3. **for Statement**

- `for` loops iterate over a range, collection, or other iterable data structures.

Example:
```rust
for number in 1..6 {
    println!("Number: {}", number);
}
```

## Conditional Expressions

### 1. **if-else Expressions**

- You can use the result of an `if-else` statement as an expression.

Example:
```rust
let age = 20;

let status = if age >= 18 {
    "adult"
} else {
    "minor"
};

println!("You are an {}.", status);
```
