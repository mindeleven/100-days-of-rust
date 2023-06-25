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

/// generics in method definitions
/// implementing method on struct Point<T>
/// generic type T has to be delared right after impl
/// doesn't have to have the same name
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// structs method signature can use different generic types
/// implementation with generic types of struct
#[allow(dead_code)]
impl<T, U> PointV2<T, U> {
    // different types for mixup method
    // method creates new point instance 
    // with a new misture of types
    fn mixup<T2, U2>(
        self,
        other: PointV2<T2, U2>
    ) -> PointV2<T, U2> {
        PointV2 {
            x: self.x,
            y: other.y,
        }
    }
}

/// specifying a constraint on a generic type
/// method only for i32 type
#[allow(dead_code)]
impl Point<i32> {
    fn distance_from_origin(&self) -> f32 {
        // method measures how far point is from (0.0, 0.0)
        // uses mathematical operation that's only available 
        // for floating point types
        // following https://doc.rust-lang.org/book/ch10-01-syntax.html here
        // and rust tells me 
        // no method named `powi` found for type `i32` in the current scoperustc
        // so I'll comment it out and leave it for now
        //(self.x.powi(2) + self.y.powi(2)).sqrt()
        1.23 // just to give something back
    }
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
#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E)
}


fn main() {
    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0, y: 4.0 };
    let int_float = PointV2 { x: 5, y: 3.22};

    println!("Point struct of type integer: {:?}", integer);
    // printing method
    println!("integer.x = {}", integer.x());

    println!("Point struct of type float: {:?}", float);
    println!("Point struct of type integer and float: {:?}", int_float);
}


