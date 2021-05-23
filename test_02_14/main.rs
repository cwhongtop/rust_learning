// 闭包作为参数的情况

fn math<F: Fn() -> i32> (op: F) -> i32 {
    op()
}
fn main() {
    let a = 2;
    let b = 3;
    assert_eq!(math(|| a + b), 5);
    println!("math(|| a + b) {}", math(|| a + b));
    assert_eq!(math(|| a * b), 6);
    println!("math(|| a * b) {}", math(|| a * b));
    println!("Hello, world!");
}
