// LRU Algorithm impl

/*
实现要点：
- 创建时要确定缓存容量
- 数据新插入或被引用时要将其优先级移至最高
- 缓存满了，再新插入数据时，淘汰掉优先级最低的数据
*/

#[macro_use]
extern crate log;

mod lru_time;
mod lru_double_list;

pub use lru_time::LruCache1;
pub use lru_double_list::LruCache2;

pub trait Lru<K, V> {
    fn put(&mut self, k: K, v: V) -> Option<V>;
    fn get(&mut self, k: &K) -> Option<&V>;
    fn len(&self) -> usize;
    fn clear(&mut self);
}

