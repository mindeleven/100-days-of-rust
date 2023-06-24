/// chapter 10, generic types, traits, and lifetimes
/// generics allow us to replace specific types 
/// with a placeholder the represents multiple types
/// 3rd example, generics in struct definitions
/// /// Generic types 3rd example, generics in struct definitions
#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0 };

    println!("Point struct of type integer: {:?}", integer);
    println!("Point struct of type float: {:?}", float);
}


