#[macro_use]
extern crate log;

use std::env;

mod host;
mod netproto;

use host::NodeId;
use std::collections::HashMap;

/*
1. 运行时输入 router nodeid, 例如节点id=1，则输入router 1, 监听端口为30000+id值，
2. 不进行参数合法性检测，相关错误处理被忽略.
3.
*/

fn main() {
    init_logger();
    let cmd: Vec<String> = env::args().collect();
    info!("cmd args: {:?}", cmd);
    let nodeid: usize = cmd[1].parse().unwrap();
    info!("local nodeid: {}", nodeid);

    let mut runtime = tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(run());
}

async fn run() {}

fn init_logger() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
}

fn create_test_connect_table() -> HashMap<NodeId, Vec<NodeId>> {
    let mut table = HashMap::new();
    let n0 = vec![1, 2];
    let n1 = vec![0, 2, 3, 4];
    let n2 = vec![1, 3, 4, 5];
    let n3 = vec![0, 1, 2, 4];
    let n4 = vec![1, 2, 3, 5];
    let n5 = vec![2, 4];

    table.insert(0, n0);
    table.insert(1, n1);
    table.insert(2, n2);
    table.insert(3, n3);
    table.insert(4, n4);
    table.insert(5, n5);

    table
}
