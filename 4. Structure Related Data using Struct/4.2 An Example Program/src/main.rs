use std::f64::consts::PI; 

fn main() {
    
    let length = 5.0    ;
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
