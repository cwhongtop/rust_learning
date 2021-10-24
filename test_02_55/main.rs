// Result<T, E> 源码实现

enum Result<T, E> {
    Ok(T),
    Err(E),
}
