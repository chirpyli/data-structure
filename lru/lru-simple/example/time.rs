#[macro_use]
extern crate log;

use lru_simple::LruCache1;
use lru_simple::Lru;

fn main() {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    info!("example of lru cache impl in lru_time.rs");

    let mut cache = LruCache1::new(3);
    cache.put("one", 1);
    cache.put("two", 2);
    cache.put("three", 3);
    cache.put("four", 4);
    let new = cache.put("three", 22);
    info!("{:?}", new);

    let key = "two";
    let v = cache.get(&key);
    info!("{:?}", v);
}