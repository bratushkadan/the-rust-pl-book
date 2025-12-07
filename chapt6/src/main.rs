use std::{fmt::format, vec};

mod ip;
mod message;
mod pattern_matching;

enum IpKind {
    V4(IPv4),
    V6(IPv6)
}

struct IPv4 {
    addr: [u8; 4]
}
impl IPv4 {
    fn str(&self) -> String {
        format!("{}.{}.{}.{}", self.addr[0], self.addr[1], self.addr[2], self.addr[3])
    }
}
struct IPv6 {
    addr: String
}

fn main() {
    let localhost = IpKind::V4(IPv4 { addr: [127, 0, 0, 1] });
    let loopback_v6 = IpKind::V6(IPv6 { addr: String::from("::1") });

    let addrs = vec![localhost, loopback_v6];

    for addr in addrs.iter() {
        match addr {
            IpKind::V4(addr) => println!("good old ipv4: {:?}", addr.str()),
            IpKind::V6(addr) => println!("cool new ipv6: {:?}", addr.addr),
        }
    }
}
