/// chapter 10, generic types, traits, and lifetimes
/// generics allow us to replace specific types 
/// with a placeholder the represents multiple types
/// 2nd example to show how to eliminate duplication
/// for functionality with different data types
fn main() {
    let number_list: Vec<i32> = vec![34, 87, 50, 100, 98, 2, 175];
    let result = largest_i32(&number_list);
    println!("Largest number in list is {}", result);

    let char_list = vec!['a', 'c', 'r', 'y', 'z', 'q', 'o'];
    let result = largest_char(&char_list);
    println!("Largest number in list is {}", result);

    // once again with the generic function
    let number_list: Vec<i32> = vec![34, 87, 50, 100, 98, 2, 175];
    let result = largest(&number_list);
    println!("Largest number in list is {}", result);

    let char_list = vec!['a', 'c', 'r', 'y', 'z', 'q', 'o'];
    let result = largest(&char_list);
    println!("Largest number in list is {}", result);

}

// function that finds the largest value in a slice of i32s
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// function that finds the largest value in a slice of chars
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// eliminating duplicating code by introducing a generic type parameter
/// when defining a function generics are placed in the signature of the function
/// not all types are sortable and can be applied to T
/// compiler suggests fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

