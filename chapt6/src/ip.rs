use std::net::{IpAddr, Ipv4Addr};

pub enum IpAddrKind {
    V4(String),
    V6(String),
}

impl IpAddrKind {
    fn localhost() -> IpAddrKind {
        IpAddrKind::V4(String::from("127.0.0.1"))
    }
    fn loobackv6() -> IpAddrKind {
        IpAddrKind::V6(String::from("::1"))
    }
}

fn _use_ip_addr_kind() {
    let home = IpAddrKind::localhost();
    let _loopback = IpAddrKind::loobackv6();

    match home {
        IpAddrKind::V4(s) => println!("yo its v4: {}", s),
        IpAddrKind::V6(s) => println!("wow its v6: {}", s),
    };

    let _localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
}