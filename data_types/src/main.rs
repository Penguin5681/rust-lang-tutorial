use std::io;
fn main() {
    // let guess :u32 = "42".parse().expect("Not a number");
    // println!("{guess}");

    let flag = true;    // Implicit Annotation
    let flag1 :bool = false;     // Explicit Annotation

    // Tuples (Can be destructured using some patterns)

    // Implicit Annotation
    let tuple1 = ("Pranav Sinha", 20, true);
    let (x, y, z) = tuple1;

    // Explicit Annotation
    let tuple2: (&str, i32, f32, bool) = ("Hello", 12, 12.3, false);
    let (a, b, c, d) = tuple2;

    println!("a = {}, x = {}", a , x);

    // Elements of tuples can be accessed using indices along with period(.)

    println!("b = {}, y = {}", tuple2.1, tuple1.1);

    // Arrays

    let num = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // println!("Month 2 = {}", months[1]);

    // Invalid Index Check for arrays (So that code doesn't panics)

    let mut idx = String::new();

    io::stdin().read_line(&mut idx).expect("Unable to get Input");

    let idx: usize = idx.trim().parse().expect("You are supposed to add a number");

    if idx > 11 {
        println!("Index Out Of Bounds!");
        return;        
    }

    println!("The Element is: {}", months[idx]);
}


// Two Subsets of data types:
/*
    Scaler and Compound

    Integer Ranges: n = 8, 16, 32, 64, 128 (-2^n -> 2^n - 1)

    Integer Literals: 
        Decimal => 98_222
        Hex => 0x1ed
        Octal => 0o567
        Binary => 0b1111_0000
        Byte => b'Z'
*/