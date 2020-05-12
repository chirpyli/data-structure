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

    pub fn route_table(&self) -> Arc<RwLock<HashMap<NodeId, RouteValue>>> {
        self.route_table.clone()
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

    pub async fn send_route_message(&self, message: &RouteMessage) {
        let dst = message.dst.clone();
        // 如果dst是本节点，返回
        if self.info.nodeid == dst {
            warn!("dst is local, return.");
            return;
        }

        let req = tonic::Request::new(message.clone());
        // 查找路由表，如果dst在路由表中，则发送给路由表中的next节点；
        let remote;
        {
            let lock_table = self.route_table.read();
            if let Some(r) = lock_table.get(&dst) {
                remote = r.clone();
            } else {
                // 如果dst不在路由表中，则随机选择一个已连接的节点，发送
                self.random_send_route_message(message).await;
                return;
            }
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

    async fn random_send_route_message(&self, message: &RouteMessage) {
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
    handler: H
}

impl<H> Server<H> {
    pub fn new(nodeid: NodeId, route_table: Arc<RwLock<HashMap<NodeId, RouteValue>>>) -> Self {
        Server {
            nodeid,
            route_table,
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
impl Network for Server {
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

        // self.send_route_message(message);

        // fixme: 临时先这么返回
        rev
    }
}

// async fn network_handle<R: Message, S: Message>(
//     handler: impl Handler<R, S>,
//     request: &[u8],
// ) -> Vec<u8> {
//     let req = match R::decode(request) {
//         Ok(r) => r,
//         Err(error) => {
//             error!("server failed to decode request. error={:?}", error);
//             panic!("server failed to decode request. error={:?}", error)
//         }
//     };

//     let res = handler.handle(req).await;
//     let res_bytes = res.encode();
//     res_bytes
// }

/*
else {
    let mut rpc = self.create_new_client(remote.next().clone()).await;
    // let hello = Hello {
    //     nodeid: self.info.nodeid(),
    // };
    // match rpc.hello(hello).await {
    //     Ok(h) => {
    //         info!("received hello from {}", h.into_inner().nodeid);
    //     }
    //     Err(e) => {
    //         error!("send hello to {} failure, {}", remote.next(), e);
    //         // todo: 从clients中删除对应项， 更新路由表
    //         return;
    //     }
    // }

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

    let mut lock = self.clients.write();
    lock.insert(remote.next().clone(), rpc);
}

*/

/*


#[derive(Clone)]
pub struct ServerImpl<R: Message, S: Message, H: Handler<R, S>> {
    handler: H,
    pub listen_addr: Option<SocketAddr>,
    whitelist: Option<BTreeSet<IpAddr>>,
    req: PhantomData<R>,
    res: PhantomData<S>,
}

impl<R: Message + Sync + 'static, S: Message + Sync + 'static, H: Handler<R, S> + 'static>
    ServerImpl<R, S, H>
{
    pub fn new(handler: H) -> Self {
        ServerImpl {
            handler,
            listen_addr: None,
            whitelist: None,
            req: PhantomData,
            res: PhantomData,
        }
    }

    pub fn new_restricted<I: IntoIterator<Item = SocketAddr>>(handler: H, white_list: I) -> Self {
        let mut set = BTreeSet::new();
        for addr in white_list {
            set.insert(addr.ip());
        }

        let server = ServerImpl {
            handler,
            listen_addr: None,
            whitelist: Some(set),
            req: PhantomData,
            res: PhantomData,
        };
        info!("whitelist :{:?}", server.whitelist);

        server
    }

    pub async fn start(&self) -> Result<(), Error> {
        let addr = self.listen_addr.clone().unwrap();
        let network;
        if let Some(ref set) = self.whitelist {
            let mut list = Vec::new();
            for ip in set {
                let addr = SocketAddr::new(ip.clone(), 0);
                list.push(addr);
            }
            network = ServerImpl::new_restricted(self.handler.clone(), list);
        } else {
            network = ServerImpl::new(self.handler.clone());
        }

        //            .add_service(NetworkServer::new(network))
        let networkservice = NetworkServer::with_interceptor(network, intercept);
        match tonic::transport::Server::builder()
            .add_service(networkservice)
            .serve(addr)
            .await
        {
            Ok(_) => info!("server started."),
            Err(e) => {
                error!("server start error. {}", e);
                return Err(Error::GrpcTransport(e));
            }
        }

        Ok(())
    }

    /// if whitelist is exist, check if remote is in list.
    fn check_in_whitelist(&self, remote: &SocketAddr) -> bool {
        let ref ip = remote.ip();
        if self.whitelist.is_some() {
            let list = self.whitelist.as_ref().unwrap();
            return list.contains(ip);
        }

        true
    }
}

#[async_trait]
impl<R, S, H> Network for ServerImpl<R, S, H>
where
    R: Message + Sync + 'static,
    S: Message + Sync + 'static,
    H: Handler<R, S> + 'static,
{
    async fn call(
        &self,
        request: tonic::Request<RawRequest>,
    ) -> Result<tonic::Response<RawResponse>, tonic::Status> {
        let remote = request.remote_addr();
        if let Some(ref addr) = remote {
            if !self.check_in_whitelist(addr) {
                info!("reject request from {}, not in white list.", addr);
                let status =
                    Status::permission_denied("local address is not in remote white list.");
                return Err(status);
            }
        } else {
            let status = Status::permission_denied(
                "local request is reject by remote server , address is none.",
            );
            return Err(status);
        }

        debug!("received request {:?} from {:?}.", request, remote);
        let req = request.into_inner();
        let req_bytes = req.req.as_slice();
        let res_bytes = network_handle(self.handler.clone(), req_bytes).await;

        let raw_res = RawResponse { res: res_bytes };
        let res = tonic::Response::new(raw_res);
        debug!("response to {:?}, {:?}", res, remote);
        Ok(res)
    }
}

async fn network_handle<R: Message, S: Message>(
    handler: impl Handler<R, S>,
    request: &[u8],
) -> Vec<u8> {
    let req = match R::decode(request) {
        Ok(r) => r,
        Err(error) => {
            error!("server failed to decode request. error={:?}", error);
            panic!("server failed to decode request. error={:?}", error)
        }
    };

    let res = handler.handle(req).await;
    let res_bytes = res.encode();
    res_bytes
}
*/ 