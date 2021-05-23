// 值表达式不能出现在位置上下文中
pub fn temp() -> i32 {
    return 1;
}

fn main() {
    let x = &temp();
    temp() = *x;
    println!("Hello, world!");
}
