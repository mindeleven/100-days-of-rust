#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // string slices
    let cat: &'static str = "Fluffy";
    println!("{}", cat);

    // string objects
    let dog = String::new();
    // needs to be marked as mutable to push something to it
    let mut dog = String::from("Max");
    println!("{}", dog);
    
    // macro
    let owner = format!("Hi, I'm {}, the owner of {}.", "Mark", dog);
    println!("{}", owner);

    // length of string 
    println!("{}", dog.len());
    // dog needs to be marked as mutable to push something to it
    dog.push(' ');
    dog.push_str("the dog");
    println!("{}", dog);
    let new_dog = dog.replace("the", "is_my");
    println!("{}", new_dog);

    // constants
    const URL: &str = "google.com"; 
    println!("{}", URL);
    
}
