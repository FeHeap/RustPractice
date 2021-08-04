fn main() {
    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", place1);   //"hello"
    let other = place2;
    //println!("{:?}", place2);   error[E0382]: borrow of moved value: `place2`
}