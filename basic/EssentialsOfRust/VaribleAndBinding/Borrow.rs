fn main() {
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p}", b);    //0xb2d62ff9a4
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d);    //[1, 2, 3, 4]
    let e = &42;
    assert_eq!(42, *e);
}