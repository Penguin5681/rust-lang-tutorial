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
        Coins::Dime => 10,
        Coins::Quarter => 25,
        Coins::Penny => 1,
        Coins::Nickel => 5,
    }
}
