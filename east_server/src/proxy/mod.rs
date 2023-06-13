use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    }
};

use crate::{
    config::{self, agent::Agent},
    connection::{self, Conns},
    plugin,
    proxy::{
        proxy_decoder::ProxyDecoder, proxy_encoder::ProxyEncoder, proxy_handler::ProxyHandler,
    },
};
use anyhow::{ Ok, Result};
use east_core::{
    bootstrap2::Bootstrap, byte_buf::ByteBuf, context::Context, message::Msg, types::TypesEnum,
};
use tokio::sync::mpsc;
use tokio::{
    io::AsyncWriteExt,
    net::TcpListener,
    select,
    sync::{
        broadcast::{self, Receiver, Sender},
        Mutex,
        RwLock
    },
};

lazy_static! {
  pub static ref last_id:AtomicU32=AtomicU32::new(1);
  pub static ref ProxyMap:Arc<Mutex<HashMap<u64,Context<ProxyMsg>>>>=Arc::new(Mutex::new(HashMap::new()));
  pub static ref IdMap:Arc<Mutex<HashMap<String,Vec<u64>>>>=Arc::new(Mutex::new(HashMap::new()));

  pub static ref Signal:Arc<Mutex<Option<mpsc::Sender<(bool,String,u16)>>>>=Arc::new(Mutex::new(None));
}

pub mod proxy_decoder;
pub mod proxy_encoder;
pub mod proxy_handler;
pub mod speed;

pub const STREAM: &str = "proxy_stream";
pub const PROXY_KEY: &str = "proxy";

#[derive(Debug)]
pub struct ProxyMsg {
    pub buf: Vec<u8>,
}

pub struct Proxy {
    port: u16,
    addr: String,
    listen: Arc<Option<TcpListener>>,
    c_rv: Arc<Mutex<Receiver<()>>>,
    c_tx: Arc<Mutex<Sender<()>>>,
    pub ids: Arc<RwLock<Vec<u64>>>,
}

impl Clone for Proxy {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            addr: self.addr.clone(),
            listen: Arc::clone(&self.listen),
            c_rv: Arc::clone(&self.c_rv.clone()),
            c_tx: Arc::clone(&self.c_tx.clone()),
            ids: Arc::clone(&self.ids),
        }
    }
}

impl std::fmt::Debug for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Proxy").field("port", &self.port).finish()
    }
}

impl Proxy {
    pub fn new(port: u16) -> Self {
        let (tx, rv) = broadcast::channel::<()>(1);
        Proxy {
            port: port,
            addr: format!("0.0.0.0:{}", port),
            listen: Arc::new(None),
            c_rv: Arc::new(Mutex::new(rv)),
            c_tx: Arc::new(Mutex::new(tx)),
            ids: Arc::new(RwLock::new(vec![])),
        }
    }
    pub fn bind_port(self) -> u16 {
        self.port
    }

    pub async fn listen(&mut self) -> Result<()> {
        let listen = TcpListener::bind(self.addr.as_str()).await?;
        self.listen = Arc::new(Some(listen));
        log::info!("代理监听：{:?}", self.addr);
        Ok(())
    }

    pub async fn close(&self) {
        self.ids.write().await.clear();
        self.c_tx.lock().await.send(()).unwrap();
    }

    pub async fn accept(&mut self, conn_id: String, ctx: Context<Msg>) -> Result<()> {
        let l = Arc::clone(&self.listen);
        let mut rv = self.c_rv.lock().await;
        let bind_port = self.port;
        if let Some(listen) = l.as_ref() {
            log::info!("开始接受代理连接{}", conn_id);
            loop {
                select! {
                _=rv.recv()=>{
                  drop(listen);
                  return Ok(())
                },
                ret=listen.accept()=>{
                    match proxy_conf_adapter(conn_id.clone(),bind_port).await{
                        Some(agent)=>{
                            let (mut stream,addr)=ret.unwrap();
                            if !agent.match_addr(addr.to_string()){
                                log::warn!("IP->{:?},不在白名单列表内,阻止连接",addr);
                                let _=stream.shutdown().await;
                            }else{
                                let id=last_id.fetch_add(1,Ordering::Relaxed) as u64;
                                // self.ids.lock().await.push(id);
                                self.ids.write().await.push(id);
                                log::info!("{:?}连接代理端口, id->{}",addr,id);
                                let mut boot=Bootstrap::build(stream, addr, ProxyEncoder::new(), ProxyDecoder::new(), ProxyHandler{ctx:ctx.clone(),id:id,conn_id:conn_id.clone(),port:bind_port});
                                if let Some(max_rate)=agent.max_rate{
                                  boot.capacity(1024);
                                  boot.set_rate_limit((max_rate*1024) as u64).await;
                                }
                                ctx.set_attribute(format!("{}_{}",STREAM,id), Box::new(Arc::new(Mutex::new(boot)))).await;
                                let conn_id=conn_id.clone();
                                let mut bf=ByteBuf::new_with_capacity(0);
                                let host=agent.target_host.clone();
                                let port=agent.target_port;
                                bf.write_string_with_u8_be_len(host);
                                bf.write_u16_be(port);
                                bf.write_u64_be(id);
                                let open_msg=Msg::new(TypesEnum::ProxyOpen,bf.available_bytes().to_vec());
                                let conn=Conns.get(conn_id.clone()).await;
                                match conn{
                                      Some(conn)=>{
                                        conn.ctx().write(open_msg).await;
                                      },
                                      None=>{
                                        ctx.remove_attribute(PROXY_KEY.into()).await;
                                        log::warn!("无{}的连接，关闭此监听",conn_id);
                                        return Ok(())
                                      }
                                }
                            }
                        },
                        None=>{
                            log::warn!("无{}配置",conn_id);
                            return Ok(())
                        }
                    }
                }
                }
            }
        }
        Ok(())
    }
}

pub fn use_plugin_match(proxy: east_plugin::proxy::Proxy, addr: String) -> bool {
    Agent {
        bind_port: proxy.bind_port,
        target_host: proxy.target_host,
        target_port: proxy.target_port,
        max_rate:None,
        whitelist: proxy.whitelist,
    }
    .match_addr(addr)
}

async fn proxy_conf_adapter(conn_id:String,bind_port: u16)->Option<Agent>{
    let conf=Arc::clone(&config::CONF);
    let plugin_result=plugin::database_plugin().await;
    match plugin_result{
        core::result::Result::Ok((plugin,_pi))=>{
            let proxy=plugin.get_proxy(bind_port);
            match proxy{
                core::result::Result::Ok((_,proxy))=>{
                    return Some(Agent{
                        bind_port: proxy.bind_port,
                        target_host: proxy.target_host,
                        target_port: proxy.target_port,
                        max_rate:None,
                        whitelist: proxy.whitelist,
                    })
                },
                Err(e)=>{
                    log::error!("获取端口{}的数据错误: {}",bind_port,e);
                }
            }
        },
        Err(_)=>{
            match conf.agent.get(&conn_id){
                Some(agents)=>{
                    let a=agents.iter().find(|&x| x.bind_port==bind_port);
                    if let Some(agent)=a{
                        return Some(agent.clone())
                    }else{
                        log::warn!("无{}配置{}",conn_id,bind_port);
                    }

                },
                None=>{
                    log::warn!("无{}配置",conn_id);
                }
            }

        }
    }
    None
}

pub async fn remove(conn_id: &String) {
    let mut id_map = IdMap.lock().await;
    let mut map = ProxyMap.lock().await;
    match id_map.get_mut(conn_id) {
        Some(v) => {
            for (_, x) in v.iter().enumerate() {
                if let Some(ctx) = map.get(x) {
                    ctx.close().await;
                    map.remove(x);
                }
            }
            id_map.remove(conn_id);
        }
        None => {
            log::warn!("无连接id->{}", conn_id);
        }
    }
}

pub async fn proxy_signal() {
    let (tx, mut rv) = mpsc::channel::<(bool, String, u16)>(1024);
    let _=Signal.lock().await.insert(tx);
    loop {
        match rv.recv().await {
            Some((is_open, id, port)) => {
                if is_open {
                    open(id, port).await;
                } else {
                    close(id, port).await;
                }
            }
            None => {}
        }
    }
}

async fn open(id: String, bind_port: u16) {
    if let Some(conn) = connection::Conns.get(id.clone()).await {
        let mut proxy = Proxy::new(bind_port);
        if let Err(e) = proxy.listen().await {
            log::error!("{:?}", e);
            return;
        }
        log::info!("开启代理转发端口->{}", bind_port);
        conn.insert(bind_port, proxy.clone()).await;
        tokio::spawn(async move {
            if let Err(e) = proxy.accept(id, conn.ctx().clone()).await {
                log::error!("{:?}", e);
            }
        });
    }
}

async fn close(id: String, bind_port: u16) {
    if let Some(conn) = connection::Conns.get(id.clone()).await {
        log::info!("关闭代理转发端口->{}", bind_port);
        conn.remove(bind_port).await;
    }
}
