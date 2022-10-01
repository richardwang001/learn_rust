fn main() {
    // let v:Vec<i32> = Vec::new();

    let mut v = Vec::new();
    v.push(1);
    let first =&v[0]; // immutable borrow occurs here
    /* cannot change v before the usage of v's immutable borrow */
    // v.push(2);
    // v.push(3);
    println!("first: {}", first); // immutable borrow later used here


    let mut v2 = vec![1, 2, 3, 4, 5];

    // This will case crash
    // let third: &i32 = &v2[20];
    // println!("The third element is {}", third);

    // This won't crash
    match v2.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // iterate vector
    for i in &mut v2 {
        *i += 10;
        println!("{}", i);}
}
