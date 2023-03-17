use std::{sync::{Arc, atomic::{AtomicUsize, Ordering, AtomicU64}}, rc::Rc, collections::HashMap};

use anyhow::{Result, Ok};
use east_core::{message::Msg, types::TypesEnum, byte_buf::ByteBuf, bootstrap::Bootstrap, context::Context};
use tokio::{net::{TcpListener, TcpStream}, io::{split, ReadHalf,WriteHalf, AsyncReadExt}, spawn, sync::{broadcast::Sender, Mutex}};

use crate::{connection::Conns, proxy::{proxy_encoder::ProxyEncoder, proxy_decoder::ProxyDecoder, proxy_handler::ProxyHandler}};

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
  w_map:HashMap<u64,WriteHalf<TcpStream>>
}

impl Proxy{
  pub fn new(addr:String)->Self{
    Proxy{
      addr:addr,
      listen:Arc::new(None),
      w_map:HashMap::new(),
    }
  }

  pub async fn listen(&mut self)->Result<()>{
    let listen=TcpListener::bind(self.addr.as_str()).await?;
    self.listen=Arc::new(Some(listen));
    println!("代理监听：{:?}",self.addr);
    Ok(())
  }

  pub async fn accept(&mut self,conn_id:String,ctx:Context<Msg>)->Result<()>{
    let l=Arc::clone(&self.listen);
    if let Some(listen)=l.as_ref(){
      println!("开始接受代理连接{}",conn_id);
      loop{
        let (stream,addr)=listen.accept().await?;
        
        // let (mut r,w)=split(stream);
        let id=last_id.load(Ordering::Relaxed);
        // self.w_map.insert(id, w);
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
        bf.write_u8_be(121u8);
        bf.write_u8_be(201u8);
        bf.write_u8_be(67u8);
        bf.write_u8_be(203u8);
        bf.write_u16_be(42880u16);
        bf.write_u64_be(id);
        let open_msg=Msg::new(TypesEnum::ProxyOpen,bf.available_bytes().to_vec());
        let conn=Conns.get(conn_id.clone()).await;
        match conn{
              Some(conn)=>{
                conn.ctx().write(open_msg).await;
              },
              None=>{}
        };
        // Bootstrap::build(stream, addr, ProxyEncoder{}, ProxyDecoder{}, ProxyHandler{ctx:ctx.clone(),id:id}).run().await.unwrap();
        // spawn(async move{
        //   loop{
        //     let mut buf=vec![0u8;64];
        //     let n=r.read(&mut buf).await.unwrap();
        //     if n==0{
        //       return;
        //     }
            
        //     let conn=Conns.get(conn_id.clone()).await;
        //     match conn{
        //       Some(conn)=>{
        //         let mut buf=buf[..n].to_vec();
        //         let mut id_bytes=id.to_be_bytes().to_vec();
        //         id_bytes.append(&mut buf);
        //         let msg=Msg::new(TypesEnum::ProxyForward,id_bytes);
        //         conn.ctx().write(msg).await;
        //       },
        //       None=>{
        //         println!("错误，不存在 {}",conn_id)
        //       }
        //     };
        //   }
        // });
      }
    }
    Ok(())
  }
}

