fn main() {
    let a: u64 = 567;
    let b: u64 = 678;
    println!("greatest common divisor of {} and {}: {}", a, b, gcd(a, b));
}

#[allow(dead_code)]
// computes greatest common divisor of two integers
// using Euclid's algorithm 
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
// runs with 'cargo test'
fn assert() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
                3 * 11);
}