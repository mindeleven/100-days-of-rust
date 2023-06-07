fn main() {
    // calling functions
    let b = highest(4, 2, 8);
    let o = other(5, 9);

    // format! will return a string type (actually a macro)
    // available option if you want to set a string type to a result
    let s = format!("{} is highest", b);

    println!("{} is highest", b);

    println!("{} is highest (output from string)", s);

    println!("other returns {}", o);
}

// syntax for a function call
fn highest(a: i32, b: u32, c: i8) -> i32 {
    // mutable result type
    let mut res = a;

    // using as keyword
    if b as i32 > res {
        res = b as i32;
    }

    if c as i32 > res {
        res = c as i32;
    }

    // return value
    res
}

// let's write another function
fn other(a: i32, b: i32) -> i32 {
    // shadowing instead of mut
    let c = a + b;
    let c = c % 4;
    let c = c / 2;
    let c = c + 1;
    c
}