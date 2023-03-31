use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::SystemTime;

use east_core::byte_buf::ByteBuf;
use east_core::context::Context;
use east_core::message::Msg;
use east_core::types::TypesEnum;
use tokio::sync::Mutex;

use crate::proxy::{Proxy, self};
use crate::handler::TIME_KEY;
use std::time::UNIX_EPOCH;

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
        let ctx=self.clone().ctx();
        let mut binds=self.bind_proxy.lock().await;
        if let Some(proxy)=binds.get(&port){
            log::info!("关闭监听端口->{}",port);
            proxy.close().await;
            let mut p_map=proxy::ProxyMap.lock().await;
            let ids=proxy.ids.lock().await;
            for (_,id) in ids.iter().enumerate(){
                let mut bf=ByteBuf::new_with_capacity(0);
                bf.write_u64_be(*id);
                let msg=Msg::new(TypesEnum::ProxyClose, bf.available_bytes().to_vec());
                ctx.write(msg).await;
                p_map.remove(id);
                log::debug!("移除转发->{}",id)
            }
        }
        binds.remove(&port);
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

    pub async fn clear_invalid_connection(&self){
        let self_conns=Arc::clone(&self.conns);
        tokio::spawn(async move{
            loop{
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                let conns=self_conns.lock().await;
                let mut r_conns=conns.clone();
                for (id,conn) in conns.iter(){
                    let conn_c=conn.clone();
                    let ctx=conn_c.ctx();
                    let t=ctx.get_attribute(TIME_KEY.into()).await;
                    let ht=t.lock().await;
                    if let Some(t)=ht.downcast_ref::<u64>(){
                        match SystemTime::now().duration_since(UNIX_EPOCH) {
                            Ok(n) => {
                                if n.as_secs()-t>TIME_OUT{
                                    log::warn!("移除心跳过期连接: {}",id);
                                    r_conns.remove(id);
                                }
                            },
                            Err(e) => log::error!("{:?}",e),
                        }
                    }
                }
            }
        });
        
    }
 
    pub async fn println(&self){
        let conns=self.conns.lock().await;
        println!("{:?}",conns);
    }
}

const TIME_OUT:u64=20;

