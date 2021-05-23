// 使用 match 表达式

fn main() {
    let mut v = vec![1,2,3,4,5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
    println!("Hello, world!");
}
