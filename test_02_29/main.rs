// 切片类型示例

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("&arr {:?}", &arr);
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    println!("&arr[1..] {:?}", &arr[1..]);
    assert_eq!(&arr[1..], [2, 3, 4, 5]);
    println!("(&arr).len() {}", (&arr).len());
    assert_eq!((&arr).len(), 5);
    println!("(&arr).is_empty() {}", (&arr).is_empty());
    assert_eq!((&arr).is_empty(), false);
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    println!("arr {:?}", arr);
    assert_eq!(arr, &[1, 7, 3]);
    let vec = vec![1, 2, 3];
    println!("&vec[..] {:?}", &vec[..]);
    assert_eq!(&vec[..], [1, 2, 3]);
}
