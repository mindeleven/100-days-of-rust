#[allow(dead_code)]
#[derive(Debug)]
pub struct Bed {
    size: i32,
    count: u32
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed), 
    Lounge
}

fn main() {
    // making use of the enum in the following code
    use self::Room::*;

    let t = Kitchen(4);
    println!("Hello from the {:?}", t);

    match t {
        Kitchen(n) => println!("The Kitchen is a room with {} rooms", n),
        d => println!("{:?}", d)
    }

    let t2 = Bedroom(Bed{size:50, count:2});
    println!("Hello from the {:?}", t2);

    match t2 {
        Kitchen(n) => println!("The Kitchen is a room with {} rooms", n),
        d => println!("{:?}", d)
    }
}
