// 闭包作为函数返回值的情况

fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}

fn main() {
    let result = two_times_impl();
    assert_eq!(result(2), 4);
    println!("result(2) {}", result(2));
    println!("Hello, world!");
}
