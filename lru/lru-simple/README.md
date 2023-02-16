# lru-simple
lru algorithm impl, see more in doc [lru](./doc/lru.md)
.



### Examples
```rust
#[macro_use]
extern crate log;

use lru_simple::LruCache2;
use lru_simple::Lru;

fn main() {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    let mut cache = LruCache2::new(3);
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
```