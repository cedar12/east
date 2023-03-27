use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use east_core::context::Context;
use east_core::message::Msg;
use tokio::sync::Mutex;

use crate::proxy::Proxy;

lazy_static! {
    pub static ref Conns:Connections=Connections::new();
}

#[derive(Clone,Debug)]
pub struct Connection{
    ctx:Context<Msg>,
    id:String,
    bind_proxy:Arc<Mutex<HashMap<u16,Proxy>>>
}

impl Connection {
    pub fn new(ctx:Context<Msg>,id:String)->Self{
        Connection { ctx:ctx, id:id ,bind_proxy:Arc::new(Mutex::new(HashMap::new()))}
    }
    pub fn ctx(self)->Context<Msg>{
        self.ctx
    }
    pub fn id(self)->String{
        self.id
    }
    pub async fn insert(&self,port:u16,p:Proxy){
        self.bind_proxy.lock().await.insert(port, p);
    }
    pub async fn remove(&self,port:u16){
        let mut binds=self.bind_proxy.lock().await;
        if let Some(proxy)=binds.get(&port){
            log::info!("关闭监听端口->{}",port);
            proxy.close().await;
            binds.remove(&port);
        }
        
    }
    pub async fn remove_all(&self){
        let mut binds=self.bind_proxy.lock().await;
        for (port,proxy) in binds.iter(){
            log::info!("关闭监听端口->{}",port);
            proxy.close().await;
        }
        binds.clear();
    }
}

#[derive(Debug)]
pub struct Connections{
    conns:Arc<Mutex<HashMap<String,Connection>>>
}


impl Connections {
    pub fn new()->Self{
        Connections { conns: Arc::new(Mutex::new(HashMap::new())) }
    }
    pub async fn insert(&self,id:String,client:Connection){
        let mut conns=self.conns.lock().await;
        conns.insert(id,client);
    }
    pub async fn remove(&self,id:String)->bool{
        let mut conns=self.conns.lock().await;
       conns.remove(&id).is_some()
    }
    pub async fn get(&self,id:String)->Option<Connection>{
        let conns=self.conns.lock().await;
        match conns.get(&id){
            Some(c)=>Some(c.clone()),
            None=>None
        }
    }
 
    pub async fn println(&self){
        let conns=self.conns.lock().await;
        println!("{:?}",conns);
    }
}

