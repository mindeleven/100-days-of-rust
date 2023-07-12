/// looping mechanisms in iterators
/// 

fn main() {
    let mut n = 0;

    // loop with break condition
    loop {
        n += 1;
        if n > 10 {
            break;
        }
        println!("Hello, {} -> world!", n);
    }
    println!("All done with loop!");

    // doing the same with while
    // no break condition needed
    while n < 20 {
        n += 1;
        println!("Hello, {} -> world!", n);
    }
    println!("All done with while!");

    // for loop with range object
    for i in 0..10 {
        println!("for loop, {} -> world", i);
    }
    println!("All done with for!");

}
