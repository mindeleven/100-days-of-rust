// enum with generic type that can take any type when implemented
#[derive(Debug)]
enum Res<T, E> {
    Thing(T),
    Error(E),
}
fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);

    println!("a = {:?}, b = {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Can't divide by zero".to_string());
    }
    Res::Thing(a / b)
}
