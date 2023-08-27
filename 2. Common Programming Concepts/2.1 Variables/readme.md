# Variables <br>
In Rust, variables are used to store and manage data. Rust has a unique and powerful system for managing variables and their lifetimes, designed to ensure memory safety and prevent common programming errors like null pointer dereferencing, buffer overflows, and data races.

Here are some key points about variables in Rust:

1.  **Immutable by Default**: In Rust, variables are immutable by default. This means that once you assign a value to a variable, you cannot change that value. This design encourages safer code by preventing accidental modifications. <br>
`let x = 42;  // Immutable variable` <br>
`x = 10; // Error: Cannot assign to `x` because it is immutable` <br>

2. **Type Annotations**: Rust can usually infer the data type of a variable from its initial value, but you can also explicitly annotate the type using a colon `:`. <br>
`let name: &str = "Alice"; // Explicit type annotation` <br>
3. **Constants**: Rust allows you to define constants using the `const` keyword. Constants must have a type annotation and cannot be mutable. <br>
`const PI: f64 = 3.14159265359; // Constant with type annotation` <br>
4. **Lifetimes**: Rust uses lifetimes to ensure that references to variables remain valid throughout their usage. Lifetimes are a way to specify the scope during which references are valid.