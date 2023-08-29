# An Example code:
## Let's create a program which calculates the area of rectangle, circle and cylinder without using structs 

```rust
use std::f64::consts::PI; 

fn main() {
    
    let length = 5.0;
    let width = 3.0;
    let radius = 4.0;
    let height = 7.0;

    let rectangle_area = area_of_rectangle(length, width);
    let circle_area = area_of_circle(radius);
    let cylinder_lateral_area = lateral_area_of_cylinder(radius, height);
    
    println!("Area of Rectangle: {:.2} square units", rectangle_area);
    println!("Area of Circle: {:.2} square units", circle_area);
    println!("Lateral Surface Area of Cylinder: {:.2} square units", cylinder_lateral_area);
}

fn area_of_rectangle(length: f64, width: f64) -> f64 {
    length * width
}


fn area_of_circle(radius: f64) -> f64 {
    PI * radius * radius
}


fn lateral_area_of_cylinder(radius: f64, height: f64) -> f64 {
    2.0 * PI * radius * height
}

```

## Now let's refactor the same code using tuples
```rust
use std::f64::consts::PI;

fn main() {
    let rectangle_dimensions = (5.0, 3.0);
    let circle_radius = 4.0;
    let cylinder_dimensions = (4.0, 7.0);

    let rectangle_area = area_of_rectangle(rectangle_dimensions);
    let circle_area = area_of_circle(circle_radius);
    let cylinder_lateral_area = lateral_area_of_cylinder(cylinder_dimensions);

    println!("Area of Rectangle: {:.2} square units", rectangle_area);
    println!("Area of Circle: {:.2} square units", circle_area);
    println!("Lateral Surface Area of Cylinder: {:.2} square units", cylinder_lateral_area);
}

fn area_of_rectangle(dimensions: (f64, f64)) -> f64 {
    let (length, width) = dimensions;
    length * width
}

fn area_of_circle(radius: f64) -> f64 {
    PI * radius * radius
}

fn lateral_area_of_cylinder(dimensions: (f64, f64)) -> f64 {
    let (radius, height) = dimensions;
    2.0 * PI * radius * height
}

```
In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.

## Let's finally refactor the code using structs: Adding more meaning
We use structs to add meaning by labeling the data. We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts
```rust
use std::f64::consts::PI;

struct Rectangle {
    length: f64,
    width: f64,
}

struct Circle {
    radius: f64,
}

struct Cylinder {
    radius: f64,
    height: f64,
}

fn main() {
    let rectangle = Rectangle { length: 5.0, width: 3.0 };
    let circle = Circle { radius: 4.0 };
    let cylinder = Cylinder { radius: 4.0, height: 7.0 };

    let rectangle_area = area_of_rectangle(&rectangle);
    let circle_area = area_of_circle(&circle);
    let cylinder_lateral_area = lateral_area_of_cylinder(&cylinder);

    println!("Area of Rectangle: {:.2} square units", rectangle_area);
    println!("Area of Circle: {:.2} square units", circle_area);
    println!("Lateral Surface Area of Cylinder: {:.2} square units", cylinder_lateral_area);
}

fn area_of_rectangle(rectangle: &Rectangle) -> f64 {
    rectangle.length * rectangle.width
}

fn area_of_circle(circle: &Circle) -> f64 {
    PI * circle.radius * circle.radius
}

fn lateral_area_of_cylinder(cylinder: &Cylinder) -> f64 {
    2.0 * PI * cylinder.radius * cylinder.height
}

```