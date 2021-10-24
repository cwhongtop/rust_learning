// 元组结构体示例
struct Color(i32, i32, i32);
fn main() {
    let color = Color(0, 1, 2);
    println!("color.0 = {}", color.0);
    assert_eq!(color.0, 0);
    println!("color.1 = {}", color.1);
    assert_eq!(color.1, 1);
    println!("color.2 = {}", color.2);
    assert_eq!(color.2, 2);
}
