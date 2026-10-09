fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    let kind = IpAddrKind::V4;
//    Basic enum syntax

    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
//    More involved syntax/usage
    
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
//    Enum options can actually take MORE information. Feels like a roundabout way to handle inheritance the way Java does and Rust SHOULD do instead of stupid traits

    enum IpAddr {
        V4(String),
        V6(String),
    }
    impl IpAddr {
        fn function(&self) {
        }
    }
    let ip = IpAddr::V4("127.0.0.1");
    ip.function();
//    Enums can also be given methods. Okay so they are literally like structs with easy inheritance got it lol.
}