fn main() {
    let mut x = 12;
    println!("The number is: {x}");
    x += 1;
    println!("The number is: {x}");

    // Constants

    const MONTHS_IN_A_YEAR :i32 = 12;
    println!("How many months in a year? {}", MONTHS_IN_A_YEAR);

    // Shadowing

    let y = 15;

    let y = 20;

    {
        let y = 25;
        println!("Value inside a scope: {y}");
    }
    println!("Value Outside the scope: {y}");

    let name = "Pranav Sinha";
    let name = name.len();      

    println!("Name: {name}, len = {name}"); // Shadowed (returns 12) 
}

