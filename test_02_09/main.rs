// 词法作用域示例
fn main() {
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    println!("1 {}", v);
    let v = "hello Rust!";
    assert_eq!(v, "hello Rust!");
    println!("2 {}", v);
    {
        let v = "hello World!";
        assert_eq!(v, "hello World!");
        println!("3 {}", v);
    }
    assert_eq!(v, "hello Rust!");
    println!("4 {}", v);
    println!("Hello, world!");
}
