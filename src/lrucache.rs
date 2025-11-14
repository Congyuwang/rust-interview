use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

use crate::linkedlist::ArrayLinkedList;

pub struct LRUCache<K: Hash + Eq + Clone, V> {
    kv: HashMap<K, CacheEntry<V>>,
    recency: ArrayLinkedList<K>,
}

struct CacheEntry<V> {
    val: V,
    ptr: usize,
}

impl<K: Hash + Eq + Clone, V> LRUCache<K, V> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            kv: HashMap::with_capacity(capacity),
            recency: ArrayLinkedList::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        match self.kv.get_mut(key) {
            Some(v) => {
                Self::update_recency(&mut self.recency, key.clone(), v);
                Some(&v.val)
            }
            None => None,
        }
    }

    pub fn put(&mut self, key: K, val: V) {
        let count = self.kv.len();
        let mut to_delete: Option<K> = None;
        match self.kv.entry(key.clone()) {
            Entry::Occupied(mut o) => {
                let cache_entry = o.get_mut();
                Self::update_recency(&mut self.recency, key, cache_entry);
                cache_entry.val = val;
            }
            Entry::Vacant(v) => {
                if count >= self.recency.capacity() {
                    to_delete = self.recency.pop_back();
                }
                if let Ok(ptr) = self.recency.push_front(key) {
                    v.insert(CacheEntry { val, ptr });
                };
            }
        };
        if let Some(k) = to_delete {
            self.kv.remove(&k);
        }
    }

    fn update_recency(recency: &mut ArrayLinkedList<K>, key: K, entry: &mut CacheEntry<V>) {
        if recency.remove(entry.ptr).is_none() {
            unreachable!("entry.ptr should always be valid.")
        }
        if let Ok(ptr) = recency.push_front(key) {
            entry.ptr = ptr;
        } else {
            unreachable!("just removed from the queue, should be able to push.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_put_and_get() {
        let mut cache = LRUCache::<i32, i32>::with_capacity(3);

        // Test basic insertion and retrieval
        cache.put(1, 100);
        cache.put(2, 200);
        cache.put(3, 300);

        assert_eq!(cache.get(&1), Some(&100));
        assert_eq!(cache.get(&2), Some(&200));
        assert_eq!(cache.get(&3), Some(&300));
        assert_eq!(cache.get(&4), None); // Non-existent key
    }

    #[test]
    fn test_lru_eviction_policy() {
        let mut cache = LRUCache::with_capacity(3);

        cache.put(1, 100);
        cache.put(2, 200);
        cache.put(3, 300);

        // Access some items to change recency
        cache.get(&1); // Makes 1 most recent
        cache.get(&2); // Makes 2 most recent

        // Add fourth item - should evict least recent (3)
        cache.put(4, 400);

        assert_eq!(cache.get(&3), None); // Should be evicted
        assert_eq!(cache.get(&1), Some(&100));
        assert_eq!(cache.get(&2), Some(&200));
        assert_eq!(cache.get(&4), Some(&400));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = LRUCache::with_capacity(2);

        cache.put(1, 100);
        cache.put(2, 200);

        // Update existing key
        cache.put(1, 150);

        assert_eq!(cache.get(&1), Some(&150)); // Updated value
        assert_eq!(cache.get(&2), Some(&200));

        // Adding new key should evict least recent (1)
        cache.put(3, 300);
        assert_eq!(cache.get(&1), None); // Evicted
        assert_eq!(cache.get(&2), Some(&200));
        assert_eq!(cache.get(&3), Some(&300));
    }

    #[test]
    fn test_capacity_one() {
        let mut cache = LRUCache::with_capacity(1);

        cache.put(1, 100);
        assert_eq!(cache.get(&1), Some(&100));

        // Add second item - should evict first
        cache.put(2, 200);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&200));
    }

    #[test]
    fn test_empty_cache() {
        let mut cache = LRUCache::<i32, i32>::with_capacity(3);

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_string_keys() {
        let mut cache = LRUCache::with_capacity(2);

        cache.put("first".to_string(), 100);
        cache.put("second".to_string(), 200);

        assert_eq!(cache.get(&"first".to_string()), Some(&100));
        assert_eq!(cache.get(&"second".to_string()), Some(&200));

        // Add third item - should evict least recent ("first")
        cache.put("third".to_string(), 300);
        assert_eq!(cache.get(&"first".to_string()), None);
        assert_eq!(cache.get(&"second".to_string()), Some(&200));
        assert_eq!(cache.get(&"third".to_string()), Some(&300));
    }

    #[test]
    fn test_complex_usage_pattern() {
        let mut cache = LRUCache::with_capacity(3);

        // Initial population
        cache.put(1, 100);
        cache.put(2, 200);
        cache.put(3, 300);

        // Access pattern: 2, 1, 4 (new)
        cache.get(&2);
        cache.get(&1);

        // Add new item - should evict 3 (least recent)
        cache.put(4, 400);

        assert_eq!(cache.get(&3), None); // Evicted
        assert_eq!(cache.get(&1), Some(&100));
        assert_eq!(cache.get(&2), Some(&200));
        assert_eq!(cache.get(&4), Some(&400));

        // Update existing and add new
        cache.put(2, 250); // Update 2
        cache.put(5, 500); // Should evict 1 (least recent after updates)

        assert_eq!(cache.get(&1), None); // Evicted
        assert_eq!(cache.get(&2), Some(&250)); // Updated value
        assert_eq!(cache.get(&4), Some(&400));
        assert_eq!(cache.get(&5), Some(&500));
    }

    #[test]
    fn test_zero_capacity() {
        let mut cache = LRUCache::<i32, i32>::with_capacity(0);

        // With zero capacity, every put should immediately evict
        cache.put(1, 100);
        assert_eq!(cache.get(&1), None);

        cache.put(2, 200);
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_large_capacity() {
        let capacity = 1000;
        let mut cache = LRUCache::with_capacity(capacity);

        // Fill the cache
        for i in 0..capacity {
            cache.put(i, i * 10);
        }

        // Verify all items are present
        for i in 0..capacity {
            assert_eq!(cache.get(&i), Some(&(i * 10)));
        }

        // Add one more item - should evict the first one (0)
        cache.put(capacity, capacity * 10);
        assert_eq!(cache.get(&0), None);
        assert_eq!(cache.get(&capacity), Some(&(capacity * 10)));
    }
}
