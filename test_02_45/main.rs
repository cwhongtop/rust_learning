// VecDeque<T>示例

use std::collections::VecDeque;

fn main() {
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);

    for i in 0..2 {
        println!("buf.get({}) = {}", i, buf[i]);
    }
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);

    for i in 0..5 {
        println!("buf.get({}) = {}", i, buf[i]);
    }
    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));
}
