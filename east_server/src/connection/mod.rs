use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::sync::Arc;
use std::time::SystemTime;

use east_core::byte_buf::ByteBuf;
use east_core::context::Context;
use east_core::message::Msg;
use east_core::types::TypesEnum;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, mpsc, RwLock};

use crate::proxy::{Proxy, self};
use crate::handler::TIME_KEY;
use std::time::UNIX_EPOCH;

lazy_static! {
    pub static ref Conns:Connections=Connections::new();
    
    pub static ref FileSignal:Arc<Mutex<Option<Sender<(String,String,String)>>>>=Arc::new(Mutex::new(None));
}

#[derive(Clone,Debug)]
pub struct Connection{
    ctx:Context<Msg>,
    id:String,
    bind_proxy:Arc<Mutex<HashMap<u16,Proxy>>>,
    pub file_sender_map:HashMap<String,Sender<()>>
}

impl Connection {
    pub fn new(ctx:Context<Msg>,id:String)->Self{
        Connection { ctx:ctx, id:id ,bind_proxy:Arc::new(Mutex::new(HashMap::new())),file_sender_map:HashMap::new()}
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
    pub async fn get_proxy(&self,port:u16)->Option<Proxy>{
        let ctx=self.clone().ctx();
        let binds=self.bind_proxy.lock().await;
        if let Some(proxy)=binds.get(&port){
            return Some(proxy.clone())
        }
        None
    }
    pub async fn remove(&self,port:u16){
        let ctx=self.clone().ctx();
        let mut binds=self.bind_proxy.lock().await;
        if let Some(proxy)=binds.get(&port){
            log::info!("关闭监听端口->{}",port);
            
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
            proxy.close().await;
        }
        binds.remove(&port);
    }
    pub async fn remove_all(&self){
        let ctx=self.clone().ctx();
        let mut binds=self.bind_proxy.lock().await;
        for (port,proxy) in binds.iter(){
            log::info!("关闭监听端口->{}",port);
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
            proxy.close().await;
        }
        binds.clear();
    }
}

#[derive(Debug)]
pub struct Connections{
    // conns:Arc<Mutex<HashMap<String,Connection>>>
    conns:Arc<RwLock<HashMap<String,Connection>>>
}


impl Connections {
    pub fn new()->Self{
        // Connections { conns: Arc::new(Mutex::new(HashMap::new())) }
        Connections { conns: Arc::new(RwLock::new(HashMap::new())) }
    }
    pub async fn insert(&self,id:String,client:Connection){
        // let mut conns=self.conns.lock().await;
        let mut conns=self.conns.write().await;
        conns.insert(id,client);
    }
    pub async fn remove(&self,id:String)->bool{
        // let mut conns=self.conns.lock().await;
        let mut conns=self.conns.write().await;
       conns.remove(&id).is_some()
    }
    pub async fn get(&self,id:String)->Option<Connection>{
        // let conns=self.conns.lock().await;
        let conns=self.conns.read().await;
        match conns.get(&id){
            Some(c)=>Some(c.clone()),
            None=>None
        }
    }

    pub async fn insert_file_sender(&self,id:String,path:String,sender:Sender<()>)->anyhow::Result<()>{
        let mut conns=self.conns.write().await;
        if let Some(c) =conns.get_mut(&id){
            c.file_sender_map.insert(path, sender);
            return Ok(())
        }
        Err(anyhow::anyhow!(""))
    }

    pub async fn clear_invalid_connection(&self){
        let self_conns=Arc::clone(&self.conns);
        tokio::spawn(async move{
            loop{
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                let mut conns=self_conns.write().await;
                let r_conns=conns.clone();
                for (id,conn) in r_conns.iter(){
                    let conn_c=conn.clone();
                    let ctx=conn_c.ctx();
                    let t=ctx.get_attribute(TIME_KEY.into()).await;
                    let ht=t.lock().await;
                    if let Some(t)=ht.downcast_ref::<u64>(){
                        match SystemTime::now().duration_since(UNIX_EPOCH) {
                            Ok(n) => {
                                if n.as_secs()-t>TIME_OUT{
                                    log::warn!("移除心跳过期连接: {}",id);
                                    conn.remove_all().await;
                                    ctx.close().await;
                                    conns.remove(id);
                                }
                            },
                            Err(e) => log::error!("{:?}",e),
                        }
                    }
                    drop(ht)
                }
                drop(conns)
            }
        });
        
    }
 
    pub async fn println(&self){
        let conns=self.conns.read().await;
        println!("{:?}",conns);
    }
}

const TIME_OUT:u64=20;



pub async fn file_signal() {
    let (tx, mut rv) = mpsc::channel::<(String,String,String)>(1024);
    let _=FileSignal.lock().await.insert(tx);
    loop {
        match rv.recv().await {
            Some((id,path,target)) => {
                // 发送文件
                match Conns.get(id.clone()).await{
                    Some(conn)=>{
                        if let Ok(metadata) = fs::metadata(path.clone()){
                            let size = metadata.len();
                            log::info!("发送文件元数据: {:?}",metadata);
                            let (tx, mut rv) = mpsc::channel::<()>(1024);
                            let ret=Conns.insert_file_sender(id.clone(),path.clone(),tx).await;
                            if let Err(_)=ret{
                                log::error!("未能设置通道");
                                return;
                            }
                            conn.ctx.set_attribute("send_file_path".into(), Box::new(path.clone())).await;
                            
                            let mut bf=ByteBuf::new_with_capacity(0);
                            bf.write_u64_be(size);
                            bf.write_str(target.as_str());
                            let msg=Msg::new(TypesEnum::FileInfo,bf.available_bytes().to_vec());
                            conn.ctx.write(msg).await;
                            log::info!("等待通知发送文件数据");
                            rv.recv().await;
                            log::info!("启动发送文件数据");
                            match read_send_file(path.as_str(),conn.ctx.clone()).await{
                                Err(e)=>{
                                    log::error!("{}",e);
                                },
                                Ok(())=>{
                                    log::info!("{}发送文件{}完成",id,path);
                                }
                            }
                        }
                    },
                    None=>{

                    }
                }
            }
            None => {}
        }
    }
}

pub async fn read_send_file(path:&str,ctx:Context<Msg>)->std::io::Result<()>{
    let mut file = File::open(path).await?;
    let mut buffer = [0; 1024*32];
    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            log::info!("读取文件结束{}",path);
            return Ok(())
        }
        let msg=Msg::new(TypesEnum::File,buffer[..n].to_vec());
        ctx.write(msg).await;
    }
}