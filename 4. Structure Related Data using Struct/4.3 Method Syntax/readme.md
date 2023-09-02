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
### <li> '<em>impl Rectangle</em>': This is the beginning of the impl block, where 'Rectangle' is the type for which you are defining methods. The defined methods would only work of the instances of the struct Rectangle. </li>
### <li> <em>'&self' </em>: This is the self parameter, representing the instance on which the method is called. It allows to access the values of the instance attributes. <br> <br> &emsp;  1. &self: Immutable borrowing (read-only access to the instance). <br> <br> &emsp; 2. &mut self: Mutable borrowing (allows modification of the instance). <br> <br> &emsp; 3. self: Consumes the instance, transferring ownership.</li> 

### <li> <em>'get_area()'</em>: This is the method signature. It specifies the method's name, parameters, and return type. </li>

## Multiple <em>'impl'</em> blocks
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

## Associated Functions

### All functions defined within an <em> 'impl' </em> block are called associated functions because they’re associated with the type named after the <em> 'impl'</em>. <br> We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the String::from function that’s defined on the String type. <br> <br> Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a special name and isn’t built into the language. For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice.

### In C++ the '::' operator is known as <em>Scope Resolution Operator</em> and has the following uses: <ol> <li> To access a global variable when there is a local variable with same name. </li> <li> To define a function outside a class. </li> <li> To access a class’s static variables.  </li></ol> <br> <br> In Rust the '::' operator (aka "Path Separator") is used used to access the associative function:

```rust
fn main() {
    let square = Rectangle::square(5);
    // sets width and length as 5 
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn square(side: u32) -> Self {
        Self {
            width: side, 
            length: side,
        }
    }
}
```

