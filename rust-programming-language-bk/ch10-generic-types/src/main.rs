/// chapter 10, generic types, traits, and lifetimes
/// generics allow us to replace specific types 
/// with a placeholder the represents multiple types
fn main() {
    // simple example to find largest number in a list
    let number_list = vec![34, 87, 50, 100, 98, 2, 175];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("Largest number in list is {}", largest);

    // now, duplicating code to find largest number in second list
    let number_list = vec![24, 67, 34, 130, 48, 52, 171];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("Largest number in both lists is {}", largest);
}
