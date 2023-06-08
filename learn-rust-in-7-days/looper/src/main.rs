fn main() {
    println!("Hello, world!");
    loop_to_10();
    a_better_way_of_loop_to_10();
    loop_to_10_with_while();
    array_loop();
    array_loop_with_macro();
    array_loop_nested_with_tag();
}

fn loop_to_10() {
    let mut n = 0;
    loop { 
        n += 1;
        println!("Hello, loop to {}!", n);
        // will run forever if not stopped by counter
        if n >= 10 {
            return;
        }
    }
}

fn a_better_way_of_loop_to_10() {
    for n in 1..10 { 
        println!("Hello from a better way to loop to {}!", n);
    }
}

fn loop_to_10_with_while() {
    let mut n = 0;

    while n < 10 {
        println!("Hello loop {} with while!", n);
        n += 1;
    }
}

// iterrating over an array of type vector
fn array_loop() {
    let mut v = Vec::new();
    v.push(4);
    v.push(7);
    v.push(9);
    
    for n in v {
        println!("{}", n);
    }
}

fn array_loop_with_macro() {
    let v = vec![4,7,8,9,10,11];

    for n in v {
        // add continue to skip a step
        if n % 2 == 0 {
            continue;
        }
        // add break to end the loop 
        if n == 11 {
            break;
        }
        println!("{}", n);
    }
}

fn array_loop_nested_with_tag() {
    let v = vec![4,7,8,9,10,11];

    'outer: for i in 0..10 {
        // we need to borrow v here
        for n in &v {
            // add break to end the loop 
            if i+n == 11 {
                break 'outer;
            }
            println!("{}", n);
        }
    }
}