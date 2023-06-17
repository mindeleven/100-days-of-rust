/// Option<T> enum definition in the standard library
/// enum Option<T>
/// { 
///     None,
///     Some(T)
/// }
/// <T> is the syntax for a generic type
/// it means the Some variant can hold any piece of data

fn main() {
    // Some and None can be used directly without the Option:: prefix
    let _some_number = Some(5);
    let _some_char = Some('e');
    // for None the overall Option<T> type needs to be annotated
    let _absent_number: Option<i32> = None;

    println!("Hello Option<T> enums!");
}
