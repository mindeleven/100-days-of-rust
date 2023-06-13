// structs can hold multiple related values
// pieces of a struct can be different types
// each piece of data will be named (unlike tuple)
#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // making use of the defined struct
    // by creating an instance and specifying concrete values
    // aka key: value pairs
    let user1 = User {
        active: true, 
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{:?}", user1);

    // accessing a specific value with dot notation
    println!("Name of user: {}", user1.username);

    // to change values by dot notation instance needs to be mutable
    let mut user2 = User {
        active: true, 
        username: String::from("someusername456"),
        email: String::from("someone456@example.com"),
        sign_in_count: 1,
    };
    println!("Email of user2: {}", user2.email);
    user2.email = String::from("someoneNewEmail@example.com");
    println!("Changed email of user2: {}", user2.email);

    let user3 = build_user(
        String::from("someusername789"), 
        String::from("someone789@example.com")
    );
    println!("User3: {}, {}", user3.username, user3.email);

    // creating instannces from other instances with struct update syntax
    let user4 = User {
        email: String::from("dixieFlatine@example.com"),
        ..user1 // must come last, remaing fields get values from corresponding fields
    };
    println!("New user with with struct update syntax: {:?}", user4);
    // values have been moved so user1 no longer can be called
    // println!("New user with with struct update syntax: {:?}", user1);
    println!("Email of user1: {}", user1.email);

}

// moving struct to build_user function
fn build_user(email: String, username: String) -> User {
    User {
        active: true, 
        username, // init shorthand syntax
        email, // init shorthand syntax
        sign_in_count: 1,
    }
}