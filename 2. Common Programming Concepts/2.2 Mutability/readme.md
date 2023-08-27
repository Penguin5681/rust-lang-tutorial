
# Understanding Mutability in Rust

In Rust, variables can be either **mutable** or **immutable**. Mutability refers to whether a variable's value can be changed after it's initially set.

## Immutable Variables

- When you create a variable in Rust without the `mut` keyword, it's **immutable** by default.
- This means that once you assign a value to an immutable variable, you can't change it.
- Think of it like a label stuck to a box; you can't change what's inside the box, only the label on the outside.

Example:
```rust
let age = 25;      // Immutable variable
age = age + 1;     // Error! Can't change an immutable variable
```

## Mutable Variables

- To create a variable that you can change, you need to use the `mut` keyword.
- Mutable variables allow you to modify their values.
- It's like having a box with a lid; you can open it and change what's inside.

Example:
```rust
let mut counter = 0;  // Mutable variable
counter = counter + 1; // This is allowed
```

## Why Does It Matter?

- Rust's emphasis on mutability helps prevent bugs.
- Immutable variables are safer because they can't be accidentally changed.
- Mutable variables are useful when you need to modify data.
- Combining both types allows for precise control over data changes, reducing the chance of errors.

## Shadowing

- Even if a variable is initially immutable, you can "shadow" it by reusing the same variable name in a new scope.
- Shadowing allows you to change the value and type of a variable within the same scope.

Example:
```rust
let age = 25;
let age = age + 1;  // Shadowing the variable
```

Mutibility is an important concept in rust to maintain precision and integrity
