// 原生指针示例

fn main() {
    let mut x = 10;
    let ptr_x = & mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    println!("x = {}", x);
    println!("y = {}", y);
    unsafe {
        *ptr_x += *ptr_y;
    }
    println!("x = {}", x);
    println!("y = {}", y);
    assert_eq!(x, 30);
}
