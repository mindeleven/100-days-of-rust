// match control flow construct
// allows to compare values against a series of patterns
// and execute code depending on the match
#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    // from 1999 to 2008 quarters were minted with state logo
    // so let's add the state as data
    Quarter(UsState)
}

// patterns that bind to values
// match arms can bind to the part of the values that match the pattern
#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // just to name a few
}

fn main() {
    println!("The value of this coin is {:?}.", value_in_cents(Coin::Nickel));
    println!("The value of this coin is {:?}.", value_in_cents(Coin::Penny));
    println!("The value of this coin is {:?}.", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // the match arms
        // each has two parts: a pattern and some code
        Coin::Penny => {
            // match arm with multiple lines of code
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // adjusted pattern according to altered enum variant
        Coin::Quarter(state) => {
            // when coin matches 
            // state variable will be bound to the value of the quarter's state
            println!("State quarter from {:?}", state);
            25
        },
    }
}
