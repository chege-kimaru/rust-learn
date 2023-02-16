enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello world"));
    m.call();

    // options
    let some_number = Some(5); // type is Option<i32>
    let some_char = Some('a');
    let absent_number: Option<i32> = None; // Have to set type of None
}

fn route(ip_kind: IpAddrKind) {}
