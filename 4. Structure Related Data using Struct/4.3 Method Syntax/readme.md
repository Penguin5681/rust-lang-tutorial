# Methods in Rust Explained!

## In Rust, methods are functions that are associated with a particular struct, enum, or trait. Methods are similar to functions in many ways, but they have a special first parameter called self that represents an instance of the struct, enum, or trait on which the method is called. Methods allow you to define behaviors and operations that are specific to the type they are associated with.

## 1. Defining Methods
### Methods are associated functions that operate on instances of the type they are defined for. Here's the basic syntax for defining methods in Rust:
```rust
impl Type {
    fn method_name(&self, parameter1: Type1, parameter2: Type2) -> ReturnType {
        // Method implementation here
    }
}
```
### Here's an example:
```rust
fn main() {
    let rectangle_1 = Rectangle {
        width: 15,
        length: 30,
    };
    println!("THe of Rectangle1 is: {} square pixels", rectangle_1.get_area());

}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.length
    }
}
```

## Let's breakdown the components of this example:
### <li> '<b>impl Rectangle</b>': This is the beginning of the impl block, where 'Rectangle' is the type for which you are defining methods. The defined methods would only work of the instances of the struct Rectangle. </li>
### <li> <b>'&self' </b>: This is the self parameter, representing the instance on which the method is called. It allows to access the values of the instance attributes. <br> <br> &emsp;  1. &self: Immutable borrowing (read-only access to the instance). <br> <br> &emsp; 2. &mut self: Mutable borrowing (allows modification of the instance). <br> <br> &emsp; 3. self: Consumes the instance, transferring ownership.</li> 

### <li> <b>'get_area()'</b>: This is the method signature. It specifies the method's name, parameters, and return type. </li>

## Multiple <b>'impl'</b> blocks
### There can be more than one 'impl' blocks for a single struct and the methods defined in different 'impl' blocks will work the same for every instance of the structure.
### Check out this example:
```rust
fn main() {
    let rectangle_1 = Rectangle {
        width: 15,
        length: 18,
    };

    let area1 = rectangle_1.get_area();

    let mut rectangle_2 = Rectangle {
        width: 20,
        length: 0,
    };

    let area2 = rectangle_2.get_area();
    rectangle_2.resize(20, 10);
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn get_area(&self) {
        self.width * self.length
    }
}

impl Rectangle {
    fn resize(&mut self, new_width: u32, new_length: u32) {
        self.width = new_width;
        self.length = new_length;
    }

    // or 

    fn also_resize(&mut self, other: Rectangle) {
        self.width = other.width;
        self.length = other.length;
    }
}
``` 
