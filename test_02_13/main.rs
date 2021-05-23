// 闭包示例

fn main() {
    let out = 42;
    // fn add(i: i32, j: i32) -> i32 { i + j + out}
    fn add(i: i32, j: i32) -> i32 { i + j }
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out};
    let closure_inferred = |i, j| i + j + out;
    let i = 1;
    let j = 2;
    println!("add(i, j) = {}", add(i, j));
    assert_eq!(3, add(i, j));
    println!("closure_annotated = {}", closure_annotated(i, j));
    assert_eq!(45, closure_annotated(i, j));
    println!("closure_inferred = {}", closure_inferred(i, j));
    assert_eq!(45, closure_inferred(i, j));
    println!("Hello, world!");
}
