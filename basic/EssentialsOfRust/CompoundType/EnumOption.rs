fn main() {
    let s: &Option<String> = &Some("hello".to_string());
    
    //Rust 2015
    match s {
        &Some(ref s) => println!("s is: {}", s),
        _ => (),
    };

    //Rust 2018
    match s {
        Some(s) => println!("s is: {}", s),
        _ => (),
    }
}