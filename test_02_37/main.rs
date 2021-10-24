// New Type 模式示例

struct Integer(u32);
type Int = i32;
fn main() {
    let int = Integer(10);
    println!("int.0 = {}", int.0);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    println!("int = {}", 10);
    assert_eq!(int, 10);
}
