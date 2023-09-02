fn main() {
    // println!("Hello, world!");
    let rectangle_1 = Rectangle {
        width: 15,
        length: 30,
    };

    println!("The area of Rectangle1 is: {} square pixels", rectangle_1.get_area());
    let mut rectangle_2 = Rectangle {
        width: 0,
        length: 12,
    };

    rectangle_2.modify(20, 5);
    println!("The area of Rectangle2 is: {} square pixels", rectangle_2.get_area());
    
    let square = Rectangle::make_square(12);
    println!("The area of rectangle3 is: {} square pixels", square.get_area());
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

impl Rectangle {
    fn modify(&mut self, new_width: u32, new_length: u32) {
        self.width = new_width;
        self.length = new_length;
    }
}

impl Rectangle {
    fn make_square(size: u32) -> Self {
        Self {
            width: size,
            length: size,
        }
    }
}