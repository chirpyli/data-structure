//! 最简单的方式实现，为每个缓存数据添加一个时间戳，用来记录最近一次访问的时间，当缓存满了的时候，淘汰数据时遍历所有缓存数据，选择时间戳最老的那个淘汰掉。
//! 这种实现的弊端是put操作O(n)，且需要额外空间保存时间数据
use std::time::Instant;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use crate::Lru;

pub struct LruCache1<K: Hash + Eq + Clone + Debug, V: Debug> {
    cache: HashMap<K, (V, Instant)>,
    capacity: usize,
}

impl<K: Hash + Eq + Clone + Debug, V: Debug> LruCache1<K, V> {
    pub fn new(cap: usize) -> Self {
        assert!(cap > 0);
        if cap == 0 {
            panic!("capacity must > 0.");
        }

        LruCache1 {
            cache: HashMap::with_capacity(cap),
            capacity: cap
        }
    }

    fn oldest(&self) -> Option<K> {
        let mut remove = None;
        let mut min = Instant::now();
        for (k, (_v, t)) in self.cache.iter() {
            if *t < min {
                min = t.clone();
                remove = Some(k.clone());
            }
        }

        remove
    }

    fn knock_out(&mut self) {
        if self.cache.len() < self.capacity {
            return;
        }

        if let Some(ref k) = self.oldest() {
            info!("current capacity<{}> is full, knock out cache key {:?}.", self.cache.len(), k);
            self.cache.remove(k);
        }

    }
}

impl<K: Hash + Eq + Clone + Debug, V: Debug> Lru<K,V> for LruCache1<K,V> {

    fn put(&mut self, k: K, v: V) -> Option<V> {
        assert!(self.cache.len() <= self.capacity);

        let now = Instant::now();
        info!("put <{:?}, ({:?}, {:?})>", k, v, now);
        if let Some((v,_t)) = self.cache.insert(k, (v, now)) {
            return Some(v);
        }

        if self.cache.len() > self.capacity {
            self.knock_out();
        }

        None
    }

    fn get(&mut self, k: &K) -> Option<&V> {
        if let Some((v, t)) = self.cache.get_mut(k) {
            *t = Instant::now();
            return Some(v)
        }

        None
    }

    fn len(&self) -> usize {
        self.cache.len()
    }

    fn clear(&mut self) {
        self.cache.clear();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity() {
        let mut cache = LruCache1::new(2);
        assert_eq!(cache.len(), 0);
        let v0 = cache.put("first", 1);
        assert!(v0 == None && cache.len() == 1);

        let v1 = cache.put("second", 2);
        assert!(v1 == None && cache.len() == 2);

        cache.put("third", 3);
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn put() {
        let mut cache = LruCache1::new(3);
        let v0 = cache.put("first", 1);
        let v1 = cache.put("second", 2);
        let v2 = cache.put("third", 3);
        assert!(v0 == None && v1 == None && v2 == None);
        let v = cache.put("first", 11);
        assert_eq!(v, Some(1));
    }

    #[test]
    fn get() {
        let mut cache = LruCache1::new(3);
        let v0 = cache.put("first", 1);
        let v1 = cache.put("second", 2);
        let v2 = cache.put("third", 3);

        let g0 = cache.get(&"first");
        assert_eq!(g0.unwrap(), &1);

        let v3 = cache.put("four", 4);
        let g1 = cache.get(&"second");
        assert_eq!(g1, None);
    }
}
