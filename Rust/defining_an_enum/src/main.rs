#![allow(unused)]
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    //enum Option<T> {
        //Some(T),
        //None,
    //}

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;

    //let four = IpAddrKind::V4;
    //let six = IpAddrKind::V6;

    //route(IpAddrKind::V4);
    //route(IpAddrKind::V6);
}

//fn route(ip_kind: IpAddrKind) {}
