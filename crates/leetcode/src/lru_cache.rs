use std::collections::{HashMap, VecDeque};

struct LRUCache {
    map: HashMap<i32, i32>,
    dq: VecDeque<i32>,
    cap: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            dq: VecDeque::new(),
            cap: capacity.try_into().unwrap(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            self.dq.retain(|&k| key != k);
            self.dq.push_front(key);
            return *self.map.get(&key).unwrap();
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.dq.retain(|&k| key != k);
            self.dq.push_front(key);
            self.map.insert(key, value);
        } else {
            if self.map.len() == self.cap {
                let last = self.dq.pop_back().unwrap();
                self.map.remove(&last);
            }

            self.map.insert(key, value);
            self.dq.push_front(key);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /**
     * Your LRUCache object will be instantiated and called as such:
     * let obj = LRUCache::new(capacity);
     * let ret_1: i32 = obj.get(key);
     * obj.put(key, value);
     */
    #[test]
    fn it_works() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
