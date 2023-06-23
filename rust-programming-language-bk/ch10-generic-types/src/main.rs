/// chapter 10, generic types, traits, and lifetimes
/// generics allow us to replace specific types 
/// with a placeholder the represents multiple types
fn main() {
    // simple example to find largest number in a list
    let number_list = vec![34, 87, 50, 100, 98, 2, 175];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    
    let result = largest(&number_list);
    println!("Largest number in list is {}", result);

    // now, duplicating code to find largest number in second list
    let number_list: Vec<i32> = vec![24, 67, 34, 130, 48, 52, 171];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    let result = largest(&number_list);
    println!("Largest number in both lists is {}", result);
}

// eliminating duplication by defining a function that creates an abstraction 
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
