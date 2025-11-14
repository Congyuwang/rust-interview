use interview::lrucache::LRUCache;

fn main() {
    const CAP: usize = 2;
    let mut cache = LRUCache::with_capacity(CAP);
    cache.put(1, 1);
    cache.put(2, 2);
    println!("{:?}", cache.get(&1)); // 1
    cache.put(3, 3); // invalidate 2
    println!("{:?}", cache.get(&3)); // 3
    println!("{:?}", cache.get(&2)); // None
    println!("{:?}", cache.get(&1)); // 1
    cache.put(4, 4); // invalidate 3
    println!("{:?}", cache.get(&3)); // None
    println!("{:?}", cache.get(&1)); // 1
}
