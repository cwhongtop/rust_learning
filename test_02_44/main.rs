// Vec<T>示例

fn main() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v = {:?}", v1);
    assert_eq!(v1, [1, 2, 3]);
    println!("v1[1] = {}", v1[1]);
    assert_eq!(v1[1], 2);
    let mut v2 = vec![0; 10];
    v2.push(10);
    for i in 0..=10 {
        println!("v2[{}] = {}", i, v2[i]);
    }
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);

    for i in 0..=2 {
        println!("v3[{}] = {}", i, v3[i]);
    }
}
