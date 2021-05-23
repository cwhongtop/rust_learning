// 数组类型示例

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [1, 2, 3];
    println!("mut_arr[0] {}", mut_arr[0]);
    assert_eq!(1, mut_arr[0]);
    mut_arr[0] = 3;
    println!("mut_arr[0] {}", mut_arr[0]);
    assert_eq!(3, mut_arr[0]);
    let init_arr = [0; 10];
    println!("init_arr[5] {}", init_arr[5]);
    assert_eq!(0, init_arr[5]);
    println!("init_arr.len() {}", init_arr.len());
    assert_eq!(10, init_arr.len());    
    println!("Hello, world!");
}
