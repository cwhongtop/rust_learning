// 使用if let 表达式

fn main() {
    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }
    println!("binary {}", binary);
    assert_eq!(binary, 1);
    println!("Hello, world!");
}
