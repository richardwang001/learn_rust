// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}*/

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("hello fn call");
    }
}

fn main() {
    /* let home = IpAddr {
         kind: IpAddrKind::V4,
         address: String::from("127.0.0.1"),
     };

     let loopback = IpAddr {
         kind: IpAddrKind::V6,
         address: String::from("::1"),
     };*/

    let loopback = IpAddrKind::V6(String::from("::1"));
    let home = IpAddrKind::V4(127, 0, 0, 1);

    // println!("{:#?}", home.address);
    // println!("{:#?}", loopback.address);

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("hello world"));
    let c = Message::ChangeColor(0, 255, 255);

    m.call();
    println!("{:#?}", q);
    println!("{:#?}", m);
    println!("{:#?}", w);
    println!("{:#?}", c);
}
