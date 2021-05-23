// bool 类型示例

fn main() {
    let x = true;
    assert_eq!(x as i32, 1);
    let y: bool = false;
    let x = 5;  // 重新定义x
    if x > 1 { println!("x is bigger than 1")};
    assert_eq!(y as i32, 0);
    println!("Hello, world!");
}
