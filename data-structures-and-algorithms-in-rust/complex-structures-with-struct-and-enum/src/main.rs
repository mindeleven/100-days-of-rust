#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red(String),
    Green(String), 
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
        fave_color: Color::Green("I like green".to_string()),
    };

    println!("Hello, people, from {}!", p.print());
    // print struct to get debug information
    println!("#[derive(Debug)] allows to get debug information: {:?}", p);

    let c = Color::Red("hi".to_string());

    match c {
        // accessing the value stored in the variant
        Color::Red(s) => {
            println!("It's red with the value '{}'", s)
        },
        // if we don't care about the value stored in the variant 
        // we can just use an underscore
        Color::Green(_) => println!("It's green"),
        Color::Blue => println!("It's blue"),
    }
}
