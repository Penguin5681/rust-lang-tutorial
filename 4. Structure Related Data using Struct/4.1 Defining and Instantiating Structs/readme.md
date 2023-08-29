# Defining and Instantiating Structs

### 1. Defining a Struct: <br> <br> 	&rarr; We define a structure using the 'struct' keyword followed by the name of the structure and its content surrounded by '{}'

### Example:
```rust
    struct student {
        name: String,
        age: u32,
    }
```

### 2. Instantiating a Struct: <br> <br> &rarr; To create an instance of a struct, use the struct's name followed by curly braces {} containing field-value pairs.

### Example:
```rust
    let student1 = student {
        name: String::from("Pranav Sinha"),
        age: 20,
    };
```
## Here are some things we can do with structs: <br> <br> 1. Accessing Struct Fields <br> 2. Updating Struct Fields <br> 3. Tuple Structs <br> 4. Unit Structs 

## 1. Accessing Struct Fields: struct fields can be accessed using dot '.' notation. <br> <br> Example: 
```rust
    fn main() {
        println!("Student Name = {}", student1.name);
        println!("Student Age = {}", student1.age);
    }
```

## 2. Updating Struct Fields: Struct fields can only be updated when its instance is mutable <br> <br> Example:
```rust
    fn main() {
        let mut student2 = student {
            name: String::from("Shreya"),
            age: 19,
        }

        student2.name = String::from("Shagun");
        println!("Name = {}". student2.name); // Shagun
    }
```

## 3. Tuple Structs: Rust also allows you to define tuple structs, which are similar to tuples but have named fields. Consider this tuple structs as an array with multiple instances <br> <br> Example: 

```rust
    fn main() {
        let black = color(0, 0, 0);
        let _white = color(255, 255, 255);

        println!("Red = {}, Green = {}, Blue = {}", black.0, black.1, black.2);
    }
    struct color(i32, i32, i32);
```

## 4. Unit Structs: You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (). <br> <br> Example:

```rust
    struct alwaysEqual;

    fn main() {
        let something = alwaysEqual;
    }
```