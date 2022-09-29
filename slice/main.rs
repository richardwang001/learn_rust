fn main() {
    let my_sring = String :: from ("Hello world");
    let wordIndex = first_word(&my_sring[..]);
    println!("words:{}",wordIndex);
    let my_sring_literal = "Hello world";
    let wordIndex = first_word(my_sring_literal);
    println!("words:{}",wordIndex);
}

fn first_word(s: &str)->&str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item ==b' ' {
            return &s[..i];
        }
    }
    &s[..]
}