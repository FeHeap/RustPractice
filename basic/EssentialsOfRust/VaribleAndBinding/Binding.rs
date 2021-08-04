fn main() {
    let a = 1;
    //a = 2;  error[E0384]: cannot assign twice to immutable variable `a`
    let mut b = 2;
    b = 3;
}