// 枚举体应用示例

enum Option {
    Some(i32),
    None,
}

fn main() {
    let s = Some(42);
    let num = s.unwrap();
    match s {
        Some(n) => println!("num is: {}", n),
        None => (),
    };
}
