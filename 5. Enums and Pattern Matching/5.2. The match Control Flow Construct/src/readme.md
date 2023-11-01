# The `match` Control Flow Construct in Rust

In Rust, the `match` expression is a powerful control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches. It is similar to a `switch` statement in other programming languages but with more flexibility and safety.

## Syntax
The `match` expression in Rust has the following syntax:

```rust
match value {
    pattern1 => {
        // Code to execute if value matches pattern1
    }
    pattern2 => {
        // Code to execute if value matches pattern2
    }
    // More patterns...
    _ => {
        // Code to execute if value matches none of the specified patterns
    }
}
```

## Example
```rust
fn main() {
    let selected_coin = Coins::Quarter;
    println!("{}", value_in_cents(selected_coin)); 
}

enum Coins {
    Penny, 
    Nickel,
    Dime, 
    Quarter,
}

fn value_in_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => {
            println!("Lucky penny!");
            1
        }
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}
```

