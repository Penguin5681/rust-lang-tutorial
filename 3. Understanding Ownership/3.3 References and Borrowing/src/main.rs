fn main() {
    // Creating a vector
    let mut data = vec![1, 2, 3, 4, 5];

    // Immutable reference (Read-Only)
    let reference = &data;
    
    // Uncommenting the line below would result in a compilation error since 'data' is borrowed immutably.
    // data.push(6); // Error: Can't modify 'data' while it's borrowed

    // Using the reference for read-only operations
    println!("Immutable Reference: {:?}", reference);

    // Mutable reference (Modification)
    let mut_reference = &mut data;

    // Modifying the data through the mutable reference is allowed
    mut_reference.push(6);
    mut_reference.pop(); // Remove the last element

    // Using the mutable reference for modifications
    println!("Mutable Reference: {:?}", mut_reference);

    // Using the original 'data' after borrowing it mutably
    println!("Data after borrowing mutably: {:?}", data);
}
