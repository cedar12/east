use std::{sync::{Arc, atomic::{AtomicUsize, Ordering, AtomicU64}}, rc::Rc, collections::HashMap};

use anyhow::{Result, Ok};
use east_core::{message::Msg, types::TypesEnum, byte_buf::ByteBuf, bootstrap::Bootstrap, context::Context};
use tokio::{net::{TcpListener, TcpStream}, io::{split, ReadHalf,WriteHalf, AsyncReadExt}, spawn, sync::{broadcast::{Sender, Receiver, self}, Mutex}, select};

use crate::{connection::Conns, proxy::{proxy_encoder::ProxyEncoder, proxy_decoder::ProxyDecoder, proxy_handler::ProxyHandler}, config};

lazy_static!{
  static ref last_id:AtomicU64=AtomicU64::new(1);
  pub static ref ProxyMap:Arc<Mutex<HashMap<u64,Context<ProxyMsg>>>>=Arc::new(Mutex::new(HashMap::new()));
}

pub mod proxy_decoder;
pub mod proxy_encoder;
pub mod proxy_handler;

pub const STREAM:&str="proxy_stream";

#[derive(Debug)]
pub struct ProxyMsg{
  pub buf:Vec<u8>
}

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
    println!("代理监听：{:?}",self.addr);
    Ok(())
  }

  pub async fn close(&self){
    self.c_tx.lock().await.send(()).unwrap();
  }

  

  pub async fn accept(&mut self,conn_id:String,ctx:Context<Msg>)->Result<()>{
    let l=Arc::clone(&self.listen);
    let mut rv=self.c_rv.lock().await;
    if let Some(listen)=l.as_ref(){
      println!("开始接受代理连接{}",conn_id);
      loop{
        select! {
          _=rv.recv()=>{
            return Ok(())
          },
          ret=listen.accept()=>{
            let (stream,addr)=ret.unwrap();
            let id=last_id.load(Ordering::Relaxed);
            if u64::MAX==id{
              last_id.store(1, Ordering::Relaxed);
            }else{
              last_id.store(id+1, Ordering::Relaxed);
            }
            println!("{:?}连接代理端口, id->{}",addr,id);
            let boot=Bootstrap::build(stream, addr, ProxyEncoder{}, ProxyDecoder{}, ProxyHandler{ctx:ctx.clone(),id:id});
            ctx.set_attribute(format!("{}_{}",STREAM,id), Box::new(Arc::new(Mutex::new(boot)))).await;
            let conn_id=conn_id.clone();
            let mut bf=ByteBuf::new_with_capacity(0);
            let conf=Arc::clone(&config::CONF);
            let host=conf.agent.target_host.clone();
            let port=conf.agent.target_port;
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
                    println!("无{}的连接，关闭此监听",conn_id);
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

