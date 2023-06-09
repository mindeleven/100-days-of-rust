fn main() {
    let s = String::from("Hello 中国");

    for c in s.chars() {
        println!("{}", c);
    }
    
    println!("-------------");

    for c in s.bytes() {
        println!("{}", c);
    }

    println!("-------------");
    
    println!("S len = {}", s.len()); // returns number of bytes, not of characters

    println!("-------------");
    println!("// printing out the numbered characters");
   // printing out the numbered characters
    for (i,c) in s.chars().enumerate() {
        println!("{} = {}", i, c);
    }

    println!("-------------");
    println!("// running through the index of the string");
    // running through the index of the string
    for (i,c) in s.char_indices() {
        println!("{} = {}", i, c);
    }

    println!("-------------");
    println!("S Len = {}", count_l(&s));
    
}

// if you write a function that takes a string as a readonly you take a &str
// it allows you to work with the string without actually changing it
// any sting can be treated as an &str because it implements deref
fn count_l(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res +=1;
        }
    }
    res
}

