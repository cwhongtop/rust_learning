// HashMap<K, V>和BTreeMap<K, V>示例

use std::collections::BTreeMap;     // 有序
use std::collections::HashMap;      // 无序

fn main() {
    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();
    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "e");
    hmap.insert(4, "d");
    bmap.insert(3, "c");
    bmap.insert(1, "a");
    bmap.insert(2, "b");
    bmap.insert(5, "e");
    bmap.insert(4, "d");
    println!("hmap {:?}", hmap);
    println!("bmap {:?}", bmap);
}
