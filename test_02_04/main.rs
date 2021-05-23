// 不变绑定与可变绑定
fn main() {
    let a = 1;
    // a = 2;   // immutable and error
    let mut b = 2;
    println!("a = {}, b = {}!", a, b);
    b = 3;  // mutable
    println!("a = {}, b = {}!", a, b);
    println!("Hello, world!");
}
