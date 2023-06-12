#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    let mut name = "John";
    for i in 1..6 {
      // parameter is passed by value 
      // which is called borrowing
      say_hi(name);
    }
    say_hello(&mut name);
    println!("{}", name);

    let greeting = say_howdy(&mut name);
    println!("fn say_howdy(): {}", greeting);
}

fn say_hi(name: &str) {
    println!("Hello {}!", name);
}

// pass parameters by reference
// we receive the parameter as a mutable
fn say_hello(name: &mut &str) {
  println!("Hello {}!", name);
  // modifying the variable where the pointer goes
  *name = "Alex";
}

// return values
// gives the type of the function that is returned
fn say_howdy(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    // return greeting;
    // can also be written without return and semicolon
    greeting
}

