// 所有权转移
fn main() {
    println!("Hello, world!");
    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", place1);
    let other = place2;
    println!("{:?}", place2);   // Err: other value used here after move

}
