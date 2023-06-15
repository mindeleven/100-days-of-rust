// example program calculating the area of a rectangel
fn main() {
    // (1) using single variables
    // let height1 = 30;
    // let width1 = 50;
    // (2) refactoring using tuples
    let rect1 = (30, 50);

    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    println!("The area of the rectangle is {} square pixels.", area(rect1));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

