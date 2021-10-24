// HashSet<K>和BTreeSet<K>示例

use std::collections::HashSet;       // 无序
use std::collections::BTreeSet;      // 有序

fn main() {
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();
    hbooks.insert("A Song of Ice and Fire");
    hbooks.insert("The Emerald City");
    hbooks.insert("The Odyssey");
    if !hbooks.contains("The Emerald City") {
        println!("We has {} books, but The Emerald City ain't one", hbooks.len());
    }
    println!("hbooks {:?}", hbooks);
    bbooks.insert("A Song of Ice and Fire");
    bbooks.insert("The Emerald City");
    bbooks.insert("The Odyssey");
    println!("bbooks {:?}", bbooks);
}
