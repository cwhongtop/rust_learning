// Option<T> 示例

fn main() {
    let s: &Option<String> = &Some("Hello".to_string());
    // Rust 2015版本
    match s {
        &Some(ref s) => println!("s is: {}", s),
        _ => (),
    };
    // Rust 2018版本
    match s {
        Some(s) => println!("s is: {}", s),
        _ => (),
    };
}
