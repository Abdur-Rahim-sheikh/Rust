#[derive(Debug)]
enum IpAddressKind {
    V4(String),
    V6(u8, u8, u8, u8),
}
#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let localhost = IpAddressKind::V4(String::from("127.0.0.1"));
    let local = IpAddressKind::V6(128, 0, 0, 1);
    println!("{localhost:?}");
}

fn route(ip_kind: IpAddressKind) {}
