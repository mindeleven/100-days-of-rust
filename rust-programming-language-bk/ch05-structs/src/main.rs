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

// tuple structs
// using tuple structs without named fields to create different types
// defining tuple structs
#[allow(dead_code)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

// unit like structs without any fields
struct AlwaysEqual;

fn main() {
    // instantiating tuple structs
    let black = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);
    println!("Color black ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point ({}, {}, {})", origin.0, origin.1, origin.2);

    // instantiating unit-like struct
    let _subject = AlwaysEqual;

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

    // creating instances from other instances with struct update syntax
    let user4 = User {
        email: String::from("dixieFlatine@example.com"),
        ..user1 // must come last, remaing fields get values from corresponding fields
    };
    println!("New user with with struct update syntax: {:?}", user4);
    // values have been moved so user1 no longer can be called
    // println!("New user with with struct update syntax: {:?}", user1);
    println!("Email of user1: {}", user1.email);

    // tuple structs
    // using tuple structs without named fields to create different types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);


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