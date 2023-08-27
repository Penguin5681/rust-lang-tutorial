# Rust Data Types Explained

In Rust, data types help us categorize and work with different kinds of information. Think of them as containers that hold specific kinds of values.

## Basic Data Types

### 1. **Integers**

- Integers are whole numbers.
- They can be either signed (positive or negative) or unsigned (only positive).

Example:
```rust
let positive_number: i32 = 42;   // Signed integer
let natural_number: u64 = 100;   // Unsigned integer
```

### 2. **Floating-Point Numbers**

- Floating-point numbers represent real numbers with a decimal point.
- Rust has two types: `f32` (single precision) and `f64` (double precision).

Example:
```rust
let pi: f64 = 3.14159265359;
```

### 3. **Booleans**

- Booleans represent true or false values.
- They are often used for conditions and decision-making.

Example:
```rust
let is_rust_fun: bool = true;
```

### 4. **Characters**

- Characters represent single Unicode characters.
- They are enclosed in single quotes.

Example:
```rust
let first_letter: char = 'A';
```

## Compound Data Types

### 5. **Tuples**

- Tuples are collections of elements of different data types.
- They have a fixed size, and elements are accessed by index.

Example:
```rust
let person: (String, i32, bool) = ("Alice".to_string(), 30, true);
```

### 6. **Arrays**

- Arrays are collections of elements of the same data type.
- They have a fixed size that's determined at compile time.

Example:
```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
```

## Custom Data Types

### 7. **Structs**

- Structs allow you to create custom data types with named fields.
- They are used for organizing related data.

Example:
```rust
struct Person {
    name: String,
    age: i32,
}

let alice = Person {
    name: "Alice".to_string(),
    age: 30,
};
```

### 8. **Enums**

- Enums represent a type that can have different values.
- They are often used for defining a limited set of options.

Example:
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```
