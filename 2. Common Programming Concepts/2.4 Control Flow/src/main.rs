fn main() {
    // Conditional Statements

    let age = 20;

    // if-else Statement
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    // match Statement
    let choice = "apple";

    match choice {
        "apple" => println!("You chose an apple."),
        "banana" => println!("You chose a banana."),
        _ => println!("Unknown choice."),
    }

    // Loops

    // loop Statement
    let mut counter = 0;

    loop {
        println!("Count: {}", counter);
        counter += 1;

        if counter >= 5 {
            break; // Exit the loop when the condition is met
        }
    }

    // while Statement
    let mut countdown = 5;

    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }

    // for Statement
    for number in 1..6 {
        println!("Number: {}", number);
    }

    // Conditional Expressions

    // if-else Expression
    let age = 20;
    let status = if age >= 18 {
        "adult"
    } else {
        "minor"
    };
    println!("You are an {}.", status);
}
