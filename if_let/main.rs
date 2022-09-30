fn main() {
    let v = Some(3);
    match v {
        Some(3) => println!("Three"),
        _ => println!("other"),
    }

    // less code way:
    if let Some(3) = v {
        println!("Three");
    } else {
        println!("other");
    }
}