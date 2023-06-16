// example program calculating the area of a rectangel
// (3) refactoring using structs: adding more meaning
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// defining methods for struct
// all functions defined in an impl block are called associated functions
impl Rectangle {
    // the first parameter has to be &self
    // &self is short for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // it's possible for a method to have the same name as a struct field
    fn width(&self) -> bool {
        self.width > 0
    }
    // methods with more parameters 
    // can this reactangle hold another rectangle?
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // it is possible to define associated functions that don't have &self as 
    // a parameter and that therefore are not functions
    // function that returns a Rectangle that's a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    // (1) using single variables
    // let height1 = 30;
    // let width1 = 50;
    // (2) refactoring using tuples
    // let rect1 = (30, 50);
    // (3) refactoring using structs: adding more meaning
    let scale = 2;
    let rect1: Rectangle = Rectangle { 
        width: dbg!(30 * scale), // getting additional info printed with debug macro
        height: 50 
    };
    let rect2: Rectangle = Rectangle { 
        width: 10,
        height: 40 
    };
    let rect3: Rectangle = Rectangle { 
        width: dbg!(60 * scale), // getting additional info printed with debug macro
        height: 45 
    };

    println!("rect1 is {:#?}", rect1);

    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    // println!("The area of the rectangle is {} square pixels.", area(rect1));
    
    // struct gets borrowed instead of taking ownership
    // main retains ownership and can continue using rect1
    // println!("The area of the rectangle is {} square pixels.", area(&rect1));

    // calling methods of Rectangle
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    
    if rect1.width() {
        println!("The rectangle has a nonzero width of {} pixel.", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let size = 9;
    println!("Creating a square with a size of {}: {:#?}", size, Rectangle::square(size));

}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct gets borrowed instead of taking ownership
// main retains ownership and can continue using rect1
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

