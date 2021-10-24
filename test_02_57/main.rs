// main 函数中返回Result<T, E>示例

use std::fs::File;
fn main() -> Result<(), sdt::io::Error> {
    let f = File::open("bat.txt")?;
    Ok(())
}
