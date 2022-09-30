#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.length > other.length && self.length > other.length
    }
    fn square(size:u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}

fn main() {
    let rect = Rectangle { width: 30, length: 40 };
    let rect2 = Rectangle { width: 30, length: 45 };
    let rect3 = Rectangle { width: 20, length: 30 };
    let square = Rectangle::square(16);
    println!("{:#?}", rect);
    println!("{}", rect.area());
    println!("1 can hold 2? ==>{}", rect.can_hold(&rect2));
    println!("1 can hold 3? ==>{}", rect.can_hold(&rect3));
    println!("{:#?}", square);
}

