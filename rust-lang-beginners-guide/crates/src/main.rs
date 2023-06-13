// crates: structuring multiple modules together inside on unit 

// binary crate: has an entry point 
// that can run the functionality in that crate (like main.rs)
// library crate: does not have an entry point 
// and simply provides functionality for other crates
// cargo is used to manage crates

// importing module using crate
// use crate::archive::arch::arch_file;
// importing module using crate giving it alias
use crate::archive::arch::arch_file as arc;

// import external crate
use rand::Rng;

// using more elements: use rand::{Rng, other};


// import module
mod archive;

fn main() {
    // arch_file("somefile.txt");
    // accessing function via alias
    arc("somefile.txt");

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
}
