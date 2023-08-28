fn main() {
    let numbers = [1, 2, 3, 4, 5];

    let slice = &numbers[1..4];

    println!("Slice: {:?}", slice);

    let mut numbers = [1, 2, 3, 4, 5];

    let mut_slice = &mut numbers[1..4];
    mut_slice[0] = 10;

    println!("Modified Numbers: {:?}", numbers);

    let text = "Hello, Rust!";
    let slice = &text[0..5];
    println!("String Slice: {}", slice);
}
