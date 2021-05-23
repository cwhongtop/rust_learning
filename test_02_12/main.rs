// const fn 示例

const fn init_len() -> usize {
    return 5;
}
fn main() {
    let arr = [0; init_len()];
    println!("Hello, world!{:?}", arr);
}
