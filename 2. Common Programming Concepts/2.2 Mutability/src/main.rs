fn main() {
    let number = 12;
    // number = 15;    Raises an error since variables in rust are immutable by default
    
    let mut another_number = 15;    // use "mut" keyword to make any variable mutable 
    another_number = 16;

    println("{}", another_number);

    let num = 16;
    let num = num + 1;     // though the variable is immutable we can shadow the variable by re-declaring it

    println!("{}", num);
}
