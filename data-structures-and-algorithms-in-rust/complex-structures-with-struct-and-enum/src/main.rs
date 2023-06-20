#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    children: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red,
    Green, 
    Blue
}

impl Person {
    /// to use Person after call to print self needs to be passed as reference
    /// otherwise ownership is transfered to method
    pub fn print(&self) -> String {
        format!(
            "name = {}, age = {}, has {} children", 
            self.name, self.age, self.children
        )
    }
}

fn main() {
    let p = Person {
        name: "Matt".to_string(),
        age: 35, 
        children: 4,
    };

    println!("Hello, people, from {}!", p.print());
    // print struct to get debug information
    println!("#[derive(Debug)] allows to get debug information: {:?}", p);

    let c = Color::Red;

    match c {
        Color::Red => println!("It's red"),
        Color::Green => println!("It's green"),
        Color::Blue => println!("It's blue"),
    }
}
