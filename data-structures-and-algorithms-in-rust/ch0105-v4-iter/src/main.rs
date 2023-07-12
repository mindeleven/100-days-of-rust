#![allow(dead_code)]
/// looping mechanisms in iterators
/// 
pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

/// giving functionality of a trait from the standard library to the Stepper
/// if you implement Iterator for an object you can use it in a for loop
impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)

    }
}

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
    

    // iterrating through the stepper
    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };
    // iterrating through the stepper with loop
    loop {
        match st.next() {
            Some(v) => println!("stepper loop {}", v),
            None => break,
        }
    }
    // iterrating through the stepper with while
    let mut st2 = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };
    while let Some(n) = st2.next() {
        println!("stepper while, {}", n);
    }
    // iterrating through the stepper with for
    let it = Stepper { curr:5, step:10, max:50 };
    for i in it {
        println!("for stepper, {} -> world", i);
    }
}
