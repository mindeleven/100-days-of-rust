// defining enum to hold IP addresses
 #[derive(Debug)]
 enum IpAddrKind {
    V4,
    V6
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // println!("Enum holds IP addresses of type four({:?}) and six({:#?})", four, six);
    route(four);
    route(six);
}

// defining function that takes any IpAddrKind enum
fn route(ip_kind: IpAddrKind) {
    println!("Ip address of type {:?}", ip_kind)
}