use super::netproto::network_client::NetworkClient;
use super::netproto::network_server::{Network, NetworkServer};
use super::netproto::{Hello, RouteAck, RouteItem, RouteMessage, RouteTable};
use async_trait::async_trait;
use parking_lot::RwLock;
use rand;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::time::{self, Duration};
use tonic::transport::Channel;

type Session = NetworkClient<Channel>;
pub type NodeId = u64;

#[derive(Clone)]
pub struct Host {
    clients: Arc<RwLock<HashMap<NodeId, Session>>>,
    route_table: Arc<RwLock<HashMap<NodeId, RouteValue>>>,
    info: HostInfo,
}

impl Host {
    pub fn new(nodeid: u64) -> Self {
        Host {
            clients: Arc::new(RwLock::new(HashMap::new())),
            route_table: Arc::new(RwLock::new(HashMap::new())),
            info: HostInfo { nodeid },
        }
    }

    pub async fn run(&self) {
        let port = 30000 + self.info.nodeid();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port as u16);
        let network = Server::new(self.info.nodeid(), self.route_table.clone(), self.clone());
        let networkservice = NetworkServer::new(network);
        tonic::transport::Server::builder()
            .add_service(networkservice)
            .serve(addr)
            .await
            .unwrap();
    }

    // 定时执行拉取邻节点路由表
    async fn random_pull_route_table(&self) {
        // fixme:先根据自身的连接列表更新路由表，正常情况还应该时有新连接或者断开一个节点时更新路由表，先不写。
        let connect_list: Vec<NodeId> = self
            .clients
            .read()
            .iter()
            .map(|(k, _v)| k.clone())
            .collect();
        let list_len = connect_list.len() as u64;
        if list_len == 0 {
            return;
        }

        {
            for ref i in connect_list {
                let mut lock = self.route_table.write();
                if !lock.contains_key(i) {
                    lock.insert(i.clone(), RouteValue::new(i.clone(), 1));
                }
            }
        }

        let mut items = Vec::new();
        for (k, v) in self.route_table.read().iter() {
            let item = RouteItem {
                dst: k.clone(),
                next: v.next.clone(),
                distance: v.distance,
            };
            items.push(item);
        }

        let route = RouteTable {
            nodeid: self.info.nodeid(),
            item: items,
        };

        // 为便于测试，随机选取一个节点，实际可随机选取k个节点
        let k = rand::random::<u64>() % list_len;
        let random_node = connect_list[k as usize];

        let mut rpc = self.clients.read().get(&k).unwrap();
        let r = rpc.pull_route_table(route).await; //fixme: 这里返回的对方节点的路由表，应该进行一次合并，这里先不合并了
    }

    // 输入连接列表测试用
    pub async fn connect_nodes(&self, table: HashMap<NodeId, Vec<NodeId>>) {
        if let Some(nodes) = table.get(self.info.nodeid_ref()) {
            for n in nodes {
                let rpc = self.create_new_client(n.clone()).await;
                let mut lock = self.clients.write();
                lock.insert(n.clone(), rpc);
            }
        }
    }

    pub async fn send_route_message(&self, message: RouteMessage) {
        let dst = message.dst.clone();
        // 如果dst是本节点，返回
        if self.info.nodeid == dst {
            warn!("dst is local, return.");
            return;
        }

        let req = tonic::Request::new(message.clone());
        // 查找路由表，如果dst在路由表中，则发送给路由表中的next节点；
        let mut random_send_flag = false;
        let remote;
        {
            let lock_table = self.route_table.read();
            if let Some(r) = lock_table.get(&dst) {
                remote = r.clone();
            } else {
                // 如果dst不在路由表中，则随机选择一个已连接的节点，发送
                // self.random_send_route_message(message).await;
                random_send_flag = true;
                return;
            }
        }

        if random_send_flag {
            self.random_send_route_message(message).await;
            return;
        }

        if let Some(c) = self.try_get_client(remote.next()) {
            info!("remote {} is connected.", remote.next());
            let mut rpc = c;
            match rpc.route_message(req).await {
                Ok(_) => {
                    info!("route message to {}", remote.next());
                }
                Err(e) => {
                    error!("route message to {} failure: {}", remote.next(), e);
                    // todo: 从clients中删除对应项， 更新路由表
                    return;
                }
            }
        }
    }

    async fn random_send_route_message(&self, message: RouteMessage) {
        let nodes: Vec<NodeId> = self
            .clients
            .read()
            .iter()
            .map(|(k, _v)| k.clone())
            .collect();
        if nodes.len() > 0 {
            let k = rand::random::<u64>() % nodes.len() as u64;
            let random_next = nodes[k as usize];

            if let Some(c) = self.try_get_client(&random_next) {
                let mut rpc = c;
                let req = tonic::Request::new(message.clone());
                match rpc.route_message(req).await {
                    Ok(_) => {
                        info!("route message to {}", random_next);
                        return;
                    }
                    Err(e) => {
                        error!("route message to {} failure: {}", random_next, e);
                        // todo: 从clients中删除对应项， 更新路由表

                        return;
                    }
                }
            }
        }

        error!("random select node failure.");
    }

    fn try_get_client(&self, remote: &NodeId) -> Option<Session> {
        if let Some(c) = self.clients.read().get(remote) {
            return Some(c.clone());
        }

        None
    }

    fn remove_peer(&self, id: &NodeId) {
        unimplemented!()
    }

    // todo:忽略参数检查及错误处理
    // 为了简化程序设计，运行时输入 router nodeid, 例如节点id=1，则输入router 1, 监听端口为30000+id值，
    async fn create_new_client(&self, nodeid: u64) -> NetworkClient<Channel> {
        let port = 30000 + nodeid;
        let remote = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port as u16);

        let mut addr = String::from("http://");
        addr.push_str(remote.to_string().as_str());
        info!("prepare connecting to {}", addr);

        let endpoint = tonic::transport::Endpoint::new(addr).unwrap();
        let endpoint = endpoint.timeout(std::time::Duration::new(3, 0));

        let conn = endpoint.connect().await.unwrap();
        info!("connected to {} success.", remote);
        NetworkClient::new(conn)
    }
}

#[async_trait]
impl Handler for Host {
    async fn handle(&self, message: RouteMessage) {
        self.send_route_message(message).await;
    }
}

#[derive(Clone)]
struct HostInfo {
    pub nodeid: u64,
}

impl HostInfo {
    pub fn nodeid(&self) -> u64 {
        self.nodeid.clone()
    }

    pub fn nodeid_ref(&self) -> &u64 {
        &self.nodeid
    }
}

#[derive(Clone)]
struct RouteValue {
    pub next: u64,
    pub distance: u64,
}

impl RouteValue {
    pub fn new(next: u64, distance: u64) -> Self {
        RouteValue { next, distance }
    }

    pub fn next(&self) -> &u64 {
        &self.next
    }

    pub fn distance(&self) -> &u64 {
        &self.distance
    }
}

#[async_trait]
pub trait Handler: Send + Sync + Clone {
    async fn handle(&self, message: RouteMessage);
}

struct Server<H> {
    nodeid: NodeId,
    route_table: Arc<RwLock<HashMap<NodeId, RouteValue>>>,
    handler: H,
}

impl<H: Handler> Server<H> {
    pub fn new(
        nodeid: NodeId,
        route_table: Arc<RwLock<HashMap<NodeId, RouteValue>>>,
        handler: H,
    ) -> Self {
        Server {
            nodeid,
            route_table,
            handler,
        }
    }

    //合并路由表，发现有新的id则添加，如果本节点的某dst的距离比(邻节点的距离+1)大，则更新本节点的该dst对应的next为邻节点id,距离为邻节点distance加1
    fn merge(&self, remote: &RouteTable) -> RouteTable {
        let mut lock = self.route_table.write();
        let remote_nodeid = remote.nodeid.clone();
        // let items = remote.item;
        let mut rev = Vec::new();

        for item in remote.item.iter() {
            if let Some(local_item) = lock.get_mut(&item.dst) {
                let dis = item.distance + 1;
                if local_item.distance > dis {
                    local_item.next = remote_nodeid;
                    local_item.distance = dis;
                }
                let route_item = RouteItem {
                    dst: item.dst.clone(),
                    next: local_item.next.clone(),
                    distance: local_item.distance.clone(),
                };
                rev.push(route_item);
            } else {
                let route_item = RouteItem {
                    dst: item.dst.clone(),
                    next: item.next.clone(),
                    distance: item.distance.clone(),
                };
                rev.push(route_item);
                lock.insert(
                    item.dst.clone(),
                    RouteValue::new(item.next.clone(), item.distance.clone()),
                );
            }
        }

        RouteTable {
            nodeid: self.nodeid.clone(),
            item: rev,
        }
    }
}

#[async_trait]
impl<H: Handler + Send + 'static> Network for Server<H> {
    async fn hello(
        &self,
        request: tonic::Request<Hello>,
    ) -> Result<tonic::Response<Hello>, tonic::Status> {
        let nodeid = self.nodeid.clone();
        let hello = Hello { nodeid };
        Ok(tonic::Response::new(hello))
    }

    /// 收到对方pull的请求，先与本节点路由表合并，再返回合并后的路由表
    async fn pull_route_table(
        &self,
        request: tonic::Request<RouteTable>,
    ) -> Result<tonic::Response<RouteTable>, tonic::Status> {
        let remote_table = request.into_inner();
        let local_table = self.merge(&remote_table);
        Ok(tonic::Response::new(local_table))
    }

    /// 收到需要路由的消息， 如果dst是本节点，则路由终止，否则查找路由表，有对应dst项则转发到next节点，如无，则
    async fn route_message(
        &self,
        request: tonic::Request<RouteMessage>,
    ) -> Result<tonic::Response<RouteAck>, tonic::Status> {
        let message = request.get_ref();
        let mut rev = Ok(tonic::Response::new(RouteAck {
            src: message.src.clone(),
            dst: message.dst.clone(),
            result: 0,
        }));

        if message.dst == self.nodeid.clone() {
            info!("received route message success from {}", message.src);
            return rev;
        }

        self.handler.handle(message.clone()).await;

        // fixme: 临时先这么返回
        rev
    }
}
