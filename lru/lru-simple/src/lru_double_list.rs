//! 双向链表实现思路： 每当新插入或者访问数据，将数据放到链表头部，当数据满了的时候，从链表尾部删除数据。
//! 哈希表+双向链表实现   主要思路是在双向链表的实现基础上，增加一个哈希表，用于索引key，使得插入和删除的复杂度降低到O(1)。
//! 具体一点就是，哈希表的K可以认为是一个索引，V 是一个双向链表，每个V都有前驱节点和后驱节点的指针。

use std::mem::MaybeUninit;
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;
use crate::Lru;


#[derive(Debug)]
struct Entry<K: Hash + Eq + Clone, V> {
    k: MaybeUninit<K>,
    v: MaybeUninit<V>,
    prev: *mut Entry<K, V>,
    next: *mut Entry<K, V>,
}

impl<K: Hash + Eq + Clone + Debug, V: Debug> Entry<K, V> {
    pub fn new(k: K, v: V) -> Self {
        Self {
            k: MaybeUninit::new(k),
            v: MaybeUninit::new(v),
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }

    pub fn new_uninit() -> Self {
        Self {
            k: MaybeUninit::uninit(),
            v: MaybeUninit::uninit(),
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }
}


pub struct LruCache2<K: Hash + Eq + Clone + Debug, V: Debug> {
    map: HashMap<K, Box<Entry<K, V>>>,
    capacity: usize,
    head: *mut Entry<K, V>,         // 最近使用的， 新增或者访问节点的时候，节点要前移到head
    tail: *mut Entry<K, V>,         // 最近最少使用的， 当capacity满了的时候，要从尾部节点删除
}

impl<K: Hash + Eq + Clone + Debug, V: Debug> LruCache2<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let mut cache = Self {
            map: HashMap::with_capacity(capacity),
            capacity,
            head: Box::into_raw(Box::new(Entry::new_uninit())),
            tail: Box::into_raw(Box::new(Entry::new_uninit())),
        };

        unsafe {
            (*cache.head).next = cache.tail;
            (*cache.tail).prev = cache.head;
        }

        cache
    }

    fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        // get后，如果key存在，则将key移到head后第一个节点
        if let Some(entry) = self.map.get_mut(k) {
            let value = &mut entry.v;
            let value_unwrap = unsafe {
               &mut *value.as_mut_ptr()
            };
            // 如果已经是唯一头部，则不进行任何操作
            if self.head == entry.prev {
                return Some(value_unwrap);
            }

            let current_ptr = unsafe {(*entry.prev).next};

            // 原有位置删除
            unsafe {
                (*entry.prev).next = entry.next;
                (*entry.next).prev = entry.prev;
            }

            // 放到head后面第一个节点的位置
            unsafe {
                (*(*self.head).next).prev = current_ptr;
                (*current_ptr).next = (*self.head).next;
                (*current_ptr).prev = self.head;
                (*self.head).next = current_ptr;
            }

            return Some(value_unwrap);
        }

        None
    }
}


impl<K: Hash + Eq + Clone + Debug, V: Debug> Lru<K, V> for LruCache2<K, V> {
    // 如果是新插入，直接插到head后面，如果是更新已有值，将原有
    fn put(&mut self, k: K, v: V) -> Option<V> {
        let new_value = format!("{:?}", v);

        // 如果是更新已有值
        if let Some(value) = self.get_mut(&k) {

            let old = std::mem::replace(value, v);
            info!("put <{:?}, {:?}>, update old value {:?}.", k, new_value, old);
            return Some(old);
        }

        // 插入新值, 如果没满，直接插入，如果满了，删除tail的前驱节点
        if self.len() >= self.capacity {
            // 容量已满，删除尾部前驱节点
            let remove_key = unsafe {
                & *(*(*self.tail).prev).k.as_ptr()
            };

            unsafe {
                let remove_ptr = (*self.tail).prev;
                (*(*remove_ptr).prev).next = self.tail;
                (*self.tail).prev = (*remove_ptr).prev;
            }

            if let Some(v) = self.map.remove(remove_key) {
                let rk = unsafe { & *v.k.as_ptr()};
                info!("capacity is full, remove key <{:?}> before insert new key {:?}.", rk, k);
            }
        }

        info!("put <{:?}, {:?}>, insert new value.", k, new_value);
        let entry = Box::new(Entry::new(k.clone(), v));
        let new_ptr = Box::into_raw(entry);

        unsafe {
            (*(*self.head).next).prev = new_ptr;
            (*new_ptr).next = (*self.head).next;
            (*new_ptr).prev = self.head;
            (*self.head).next = new_ptr;
        }

        self.map.insert(k, unsafe { Box::from_raw(new_ptr) });

        None
    }

    fn get(&mut self, k: &K) -> Option<&V> {
        // get后，如果key存在，则将key移到head后第一个节点
        if let Some(entry) = self.map.get_mut(k) {
            let ref value = entry.v;
            let value_unwrap = unsafe {
                &*value.as_ptr()
            };
            // 如果已经是唯一头部，则不进行任何操作
            if self.head == entry.prev {
                return Some(value_unwrap);
            }

            let current_ptr = unsafe {(*entry.prev).next};

            // 原有位置删除
            unsafe {
                (*entry.prev).next = entry.next;
                (*entry.next).prev = entry.prev;
            }

            // 放到head后面第一个节点的位置
            unsafe {
                (*(*self.head).next).prev = current_ptr;
                (*current_ptr).next = (*self.head).next;
                (*current_ptr).prev = self.head;
                (*self.head).next = current_ptr;
            }

            return Some(value_unwrap);
        }

        None
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn clear(&mut self) {
        self.map.clear();
        unsafe {
            (*self.head).next = self.tail;
            (*self.tail).prev = self.head;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity() {
        let mut cache = LruCache2::new(2);
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
        let mut cache = LruCache2::new(3);
        let v0 = cache.put("first", 1);
        let v1 = cache.put("second", 2);
        let v2 = cache.put("third", 3);
        assert!(v0 == None && v1 == None && v2 == None);
        let v = cache.put("first", 11);
        assert_eq!(v, Some(1));
    }

    #[test]
    fn get() {
        let mut cache = LruCache2::new(3);
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
