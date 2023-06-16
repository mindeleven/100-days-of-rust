// defining enum to hold IP addresses
 #[derive(Debug)]
 enum IpAddrKind {
    V4,
    V6,
}

// struct to store IP addresses
 #[derive(Debug)]
 struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // println!("Enum holds IP addresses of type four({:?}) and six({:#?})", four, six);
    route(four);
    route(six);

    // instantiating struct to store IP adresses
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("IP address home: {:#?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback: {:#?}", loopback);
    
}

// defining function that takes any IpAddrKind enum
fn route(ip_kind: IpAddrKind) {
    println!("Ip address of type {:?}", ip_kind)
}