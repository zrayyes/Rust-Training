enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    println!("123");
}

fn route(ip_type: IpAddrKind) {}
