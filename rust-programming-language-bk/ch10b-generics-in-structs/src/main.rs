/// chapter 10, generic types, traits, and lifetimes
/// generics allow us to replace specific types 
/// with a placeholder the represents multiple types
/// 3rd example, generics in struct definitions
/// 
/// using generics in a struct definition
/// defininition with only one generic type
/// struct is generic over some type T
#[derive(Debug)]
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

// defining a struct with different generic types
#[derive(Debug)]
#[allow(dead_code)]
struct PointV2<T, U> {
    x: T,
    y: U,
}

/// generics in enum definitions
/// Option enum of standard library
/// Enum with generic type T
#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None
}
/// Result enum of standard library
/// Enum with multiple generic types
/// generic over two types T and E
enum Result<T, E> {
    Ok(T),
    Err(E)
}


fn main() {
    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0 };
    let int_float = PointV2 { x: 5, y: 3.22};

    println!("Point struct of type integer: {:?}", integer);
    println!("Point struct of type float: {:?}", float);
    println!("Point struct of type integer and float: {:?}", int_float);
}


