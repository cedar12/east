use std::{sync::{Arc, atomic::{AtomicUsize, Ordering, AtomicU64}}, rc::Rc, collections::HashMap};

use anyhow::{Result, Ok,anyhow};
use east_core::{message::Msg, types::TypesEnum, byte_buf::ByteBuf, bootstrap::Bootstrap, context::Context};
use tokio::{net::{TcpListener, TcpStream}, io::{split, ReadHalf,WriteHalf, AsyncReadExt}, spawn, sync::{broadcast::{Sender, Receiver, self}, Mutex}, select};

use crate::{connection::Conns, proxy::{proxy_encoder::ProxyEncoder, proxy_decoder::ProxyDecoder, proxy_handler::ProxyHandler}, config};

lazy_static!{
  static ref last_id:AtomicU64=AtomicU64::new(1);
  pub static ref ProxyMap:Arc<Mutex<HashMap<u64,Context<ProxyMsg>>>>=Arc::new(Mutex::new(HashMap::new()));
  pub static ref IdMap:Arc<Mutex<HashMap<String,Vec<u64>>>>=Arc::new(Mutex::new(HashMap::new()));
}

pub mod proxy_decoder;
pub mod proxy_encoder;
pub mod proxy_handler;

pub const STREAM:&str="proxy_stream";
pub const PROXY_KEY:&str="proxy";

#[derive(Debug)]
pub struct ProxyMsg{
  pub buf:Vec<u8>
}

#[derive(Clone)]
pub struct Proxy{
  addr:String,
  listen:Arc<Option<TcpListener>>,
  c_rv:Arc<Mutex<Receiver<()>>>,
  c_tx:Arc<Mutex<Sender<()>>>,
}

impl Proxy{
  pub fn new(addr:String)->Self{
    let (tx,rv)=broadcast::channel::<()>(1);
    Proxy{
      addr:addr,
      listen:Arc::new(None),
      c_rv:Arc::new(Mutex::new(rv)),
      c_tx:Arc::new(Mutex::new(tx))
    }
  }

  pub async fn listen(&mut self)->Result<()>{
    let listen=TcpListener::bind(self.addr.as_str()).await?;
    self.listen=Arc::new(Some(listen));
    log::info!("代理监听：{:?}",self.addr);
    Ok(())
  }

  pub async fn close(&self){
    self.c_tx.lock().await.send(()).unwrap();
  }

  

  pub async fn accept(&mut self,conn_id:String,ctx:Context<Msg>)->Result<()>{
    ctx.set_attribute(PROXY_KEY.into(), Box::new(self.clone())).await;
    let l=Arc::clone(&self.listen);
    let mut rv=self.c_rv.lock().await;
    let self_addr=self.addr.clone();
    if let Some(listen)=l.as_ref(){
      log::info!("开始接受代理连接{}",conn_id);
      loop{
        select! {
          _=rv.recv()=>{
            return Ok(())
          },
          ret=listen.accept()=>{
            let conf=Arc::clone(&config::CONF);
            match conf.agent.get(&conn_id){
              Some(agents)=>{
                let (stream,addr)=ret.unwrap();
                let a=agents.iter().find(|&x| format!("0.0.0.0:{}",x.bind_port).to_string() == self_addr);
                if let Some(agent)=a{
                  if !agent.match_addr(addr.to_string()){
                    return Err(anyhow!("IP->{:?},不在白名单列表内,阻止连接",addr))
                  }

                  let id=last_id.load(Ordering::Relaxed);
                  if u64::MAX==id{
                    last_id.store(1, Ordering::Relaxed);
                  }else{
                    last_id.store(id+1, Ordering::Relaxed);
                  }
                  log::info!("{:?}连接代理端口, id->{}",addr,id);
                  let boot=Bootstrap::build(stream, addr, ProxyEncoder{}, ProxyDecoder{}, ProxyHandler{ctx:ctx.clone(),id:id,conn_id:conn_id.clone()});
                  ctx.set_attribute(format!("{}_{}",STREAM,id), Box::new(Arc::new(Mutex::new(boot)))).await;
                  let conn_id=conn_id.clone();
                  let mut bf=ByteBuf::new_with_capacity(0);
                  let host=agent.target_host.clone();
                  let port=agent.target_port;
                  // bf.write_u8_be(121);
                  // bf.write_u8_be(201);
                  // bf.write_u8_be(67);
                  // bf.write_u8_be(203);
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
            };
            
          }
          }
        }
        
    }
    Ok(())
  }
}


pub async fn remove(conn_id:&String){
  let mut id_map=IdMap.lock().await;
  let mut map=ProxyMap.lock().await;
  match id_map.get_mut(conn_id){
    Some(v)=>{
      for (_,x) in v.iter().enumerate(){
        if let Some(ctx)=map.get(x){
          ctx.close().await;
          map.remove(x);
        }
      }
      id_map.remove(conn_id);
    },
    None=>{
      log::warn!("无连接id->{}",conn_id);
    }
  }
}