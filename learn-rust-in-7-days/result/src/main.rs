use std::collections::HashMap;
use std::env::args;
// result and option types
// result and option are both enums
fn main() {
    // if a key doesn't exist in the HashMap
    // then what should our return be?
    // answer: key should be an option
    let mut hm = HashMap::new();
    // HashMap can work out it's type from the first contents we put in it
    hm.insert(3, "Hello");
    hm.insert(5, "world");

    // getting pointer to key
    // let r1 = hm.get(&3);
    // what gets returned is an option
    // either it has something or it has not
    // two possible values: Some(v) or None 

    let r = match hm.get(&3) {
        Some(v) => v,
        _ => "NOTHING" 
    };
    println!("{}", r);

    // options come with a number of methods that allow us to handle them
    // according to the needs of our program
    // using unwrap instead of match
    // if there's Some it'll get returned
    let r2: &&str = hm.get(&3).unwrap();
    println!("{}", r2);

    // trying to get a nonexistent key
    /***
    let r3: &&str = hm.get(&4).unwrap();
    // program will panic
    // panicked at 'called `Option::unwrap()` on a `None` value'
    println!("{}", r3);
    ***/
    // using unwrap_or instead
    // default needs pointer because get always returns a pointer
    let r3: &&str = hm.get(&4).unwrap_or(&"No_string");
    // program will panic
    // panicked at 'called `Option::unwrap()` on a `None` value'
    println!("{}", r3);

    // if you parse a string to a number and the type will not pass
    // you'll get an error
    match "3".parse::<f32>() {
        Ok(v) => println!("OK, {}", v),
        Err(e) => println!("Error, {}", e)
    } // returns ok
    match "3s".parse::<f32>() {
        Ok(v) => println!("OK, {}", v),
        Err(e) => println!("Error, {}", e)
    } // returns error

    match get_arg(3) {
        Ok(v) => println!("OK, {}", v),
        Err(e) => println!("Error, {}", e)
    } 

}

fn get_arg(n:usize) -> Result<String, String> {
    for (i,a) in args().enumerate() {
        if i == n {
            return Ok(a)
        }
    }
    Err("Not enough arguments".to_string())
} // returns error but will return ok when called with arguments 
// like "cargo run -- h i p d"
