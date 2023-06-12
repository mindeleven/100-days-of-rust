// if you want to use struct outside of project make it public with pub
// pub struct User {}
#[derive(Debug)]
struct User {
    name:     String,
    age:      i32,
    height:   i32,
    shoesize: i32
}

// adding method to struct to print User
impl User {
    // taking a pointer to self as a parameter 
    fn simple_string(&self) -> String {
        format!("{} -- {} -- {}cm -- shoe:{}", self.name, self.age, self.height, self.shoesize)
    }

    fn grow(&mut self, h: i32) {
        self.height += h;
    }

    fn die(self) {
        println!("Dead {}", self.simple_string());
    }
}

fn main() {
    let mut u = User {
        name:     "Matt".to_string(),
        age:      33,
        height:   250,
        shoesize: 10
    };
    
    // #[derive(Debug)] lets us print the struct
    println!("User is {:?}", u);

    // printing with implemented method
    println!("User is {}", u.simple_string());

    u.grow(20);
    println!("User after growth is {}", u.simple_string());

    u.die(); // die consumes self; struct cannot be altered thereafter
    //u.grow(10);
}
