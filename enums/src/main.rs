enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    println!("{}");
}
