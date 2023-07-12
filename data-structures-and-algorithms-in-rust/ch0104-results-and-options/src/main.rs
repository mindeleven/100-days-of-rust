// enum with generic type that can take any type when implemented
#[derive(Debug)]
enum Res<T, E> {
    Thing(T),
    Error(E),
}

// Option enum
// is used when something could be there but might not be there
pub enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);

    match a {
        Res::Thing(v) => println!("val = {}", v),
        // underscore matches every other case
        _ => {}
    }

    // if you only care about one case the following pattern can be used too
    if let Res::Thing(v) = a { // if we can treat as a Res::Thing
        // then this block will be executed
        println!("val (from if let) = {}", v);
    }

    println!("a = {:?}, b = {:?}", a, b);

    // using the result enum from the standard library instead
    let a2 = divide_v2(10, 5);
    let b2 = divide_v2(10, 0);

    match a2 {
        Result::Ok(v) => println!("val = {}", v),
        // underscore matches every other case
        _ => {}
    }

    // if you only care about one case the following pattern can be used too
    if let Result::Ok(v) = a2 { // if we can treat as a Res::Thing
        // then this block will be executed
        println!("val (from if let) = {}", v);
    }

    println!("a2 = {:?}, b2 = {:?}", a2, b2);
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        // Rust doesn't use execptions like other languages do
        // just returns a value that says yes or no
        return Res::Error("Can't divide by zero".to_string());
    }
    Res::Thing(a / b)
}

// using the result enum from the standard library instead
fn divide_v2(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        // Rust doesn't use execptions like other languages do
        // just returns a value that says yes or no
        return Result::Err("Can't divide by zero".to_string());
    }
    Result::Ok(a / b)
}
