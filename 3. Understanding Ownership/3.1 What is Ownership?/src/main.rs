fn main() {
    // Ownership and Move Semantics

    let original = String::from("Hello"); // 'original' is the owner
    let new_variable = original; // 'original' is moved to 'new_variable'

    // Trying to use 'original' here would result in an error
    // println!("Original: {}", original); // Uncommenting this line will produce an error

    // Borrowing and References

    let mut data = vec![1, 2, 3]; // 'data' is the owner

    // Immutable Reference (Read-Only)
    let reference = &data; // Immutable reference to 'data'

    // Uncommenting the line below would result in an error since 'data' is borrowed immutably.
    // data.push(4); // Error: Can't modify 'data' while it's borrowed

    // Mutable Reference (Modification)
    let mut_reference = &mut data; // Mutable reference to 'data'
    mut_reference.push(4); // Modifying 'data' through the mutable reference is allowed

    println!("Data after modification: {:?}", data); // Prints: Data after modification: [1, 2, 3, 4]
}
