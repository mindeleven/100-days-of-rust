// match control flow construct
#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    let picked_coin = Coin::Nickel;
    println!("The value of this coin is {}.", value_in_cents(picked_coin));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
