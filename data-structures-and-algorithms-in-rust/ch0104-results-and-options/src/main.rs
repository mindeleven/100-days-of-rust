// enum with generic type that can take any type when implemented
#[derive(Debug)]
enum Res<T, E> {
    Thing(T),
    Error(E),
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
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        // Rust doesn't use execptions like other languages do
        // just returns a value that says yes or no
        return Res::Error("Can't divide by zero".to_string());
    }
    Res::Thing(a / b)
}
