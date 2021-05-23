// 元组示例

fn move_coords( x: (i32, i32) ) -> (i32, i32) {
    (x.0 + 1, x.1 + 1)
}

fn main() {
    let tuple : (&'static str, i32, char) = ("hello", 5, 'c');
    println!("tuple.0 = {}", tuple.0);
    assert_eq!(tuple.0, "hello");
    println!("tuple.1 = {}", tuple.1);
    assert_eq!(tuple.1, 5);
    println!("tuple.2 = {}", tuple.2);
    assert_eq!(tuple.2, 'c');
    let coords = (0, 1);

    let result = move_coords(coords);
    println!("result = {:?}", result);
    assert_eq!(result, (1, 2));
    let (x, y) = move_coords(coords);
    println!("x = {}", x);
    assert_eq!(x, 1);
    println!("y = {}", y);
    assert_eq!(y, 2);
}
