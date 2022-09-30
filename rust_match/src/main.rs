/*#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        },
    }
}*/

fn main() {
    // println!("Quarter iS {} cents", value_in_cents(Coin::Quarter));
    /*let coin = Coin::Quarter(UsState::Alabama);
    println!("{:#?}", value_in_cents(coin));*/

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:#?}", six);
    println!("{:#?}", none);

    /*let v = 0u8;
    match v {
        1 => println!("one"),
        2 => println!("two"),
        2 => println!("three"),
        _ => {}
    }*/
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
