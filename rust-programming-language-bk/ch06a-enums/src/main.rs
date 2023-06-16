// defining enum to hold IP addresses
// instead of using a struct to store IP adresses
// the enum can directly be used to store data
 #[derive(Debug)]
 enum IpAddrKind {
    V4,
    V6,
}

// an enum can directly be used to store data
// each type can have different types of associated data
 #[derive(Debug)]
 enum IpAddrKindV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// another enum example with a wide variety of types
// four variants with different types
// defining an enum like this is similar to defining different kind of struct definitions
 #[derive(Debug)]
 #[allow(dead_code)]
 enum Message {
    Quit, // no data associated with it at all
    Move {x: i32, y: i32}, // has named fields like struct
    Write(String), // includes single string
    ChangeColor(i32, i32, i32), // includes three i32 values
}

// defining a method on an enum
impl Message {
    fn call(&self) {
        println!("Don't call us, we call you")
    }
}

// struct to store IP addresses
//  #[derive(Debug)]
//  struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // println!("Enum holds IP addresses of type four({:?}) and six({:#?})", four, six);
    route(four);
    route(six);

    // instantiating struct to store IP adresses
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddrKindV2::V4(127, 0, 0, 1);
    println!("IP address home: {:#?}", home);

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    let loopback: IpAddrKindV2 = IpAddrKindV2::V6(String::from("::1"));
    println!("loopback: {:#?}", loopback);

    let m = Message::Write(String::from("Hello"));
    println!("Message written: {:#?}", m);
    m.call();

}

// defining function that takes any IpAddrKind enum
fn route(ip_kind: IpAddrKind) {
    println!("Ip address of type {:?}", ip_kind)
}