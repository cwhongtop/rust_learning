// 函数作为返回值的情况
fn is_true() -> bool { true }
fn true_maker() -> fn() -> bool { is_true }
fn main() {
    assert_eq!(true_maker() (), true);
    println!("Hello, world!");
}
