// str 字符串示例

fn main() {
    let truth: &'static str = "Rust是一门优雅的语言";
    println!("truth {}", truth);
    let ptr = truth.as_ptr();
    println!("ptr {:p}", ptr);
    let len = truth.len();
    println!("len {}", len);
    assert_eq!(28, len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice);
        println!("slice {:?}", slice);
    };

}
