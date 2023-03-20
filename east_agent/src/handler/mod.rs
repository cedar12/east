use std::sync::Arc;

use east_core::{handler::Handler, message::Msg, context::Context, types::TypesEnum, byte_buf::ByteBuf, bootstrap::Bootstrap};
use tokio::{net::TcpStream, spawn};

use crate::{proxy::{proxy_encoder::ProxyEncoder, proxy_decoder::ProxyDecoder, proxy_handler::ProxyHandler, self}, config};

lazy_static!{
  pub static ref CTX:Option<Context<Msg>>=None;
}

pub struct AgentHandler {}

#[async_trait::async_trait]
impl Handler<Msg> for AgentHandler {
    async fn read(&self, ctx: &Context<Msg>, msg: Msg) {
        println!("read len {:?}", msg.data.len());
        match msg.msg_type{
          TypesEnum::Auth=>{},
          TypesEnum::ProxyOpen=>{
            spawn(proxy_open(msg,ctx.clone()));
          },
          TypesEnum::ProxyForward=>{
            proxy_forward(msg).await;
          },
          TypesEnum::ProxyClose=>{
            let mut bf=ByteBuf::new_from(&msg.data);
            let id=bf.read_u64_be();
            // proxy::ProxyMap.lock().await.remove(&id);
            let map=proxy::ProxyMap.lock().await;
            println!("agent close {} {:?} ",id, map);
            match map.get(&id){
              Some(ctx)=>{
                ctx.close().await;
                // proxy::ProxyMap.lock().await.remove(&id);
                println!("agent close {} {:?} ",id, map);
              }, 
              None=>{
                println!("{} 不存在",id)
              }
            }
           
           
            
          }
        }
        // ctx.write(m).await;
    }
    async fn active(&self, ctx: &Context<Msg>) {
        println!("active {:?}", ctx.addr());
        let conf=Arc::clone(&config::CONF);
        let id=conf.id.clone();
        let msg=Msg::new(TypesEnum::Auth,id.as_bytes().to_vec());
        ctx.write(msg).await;
    }
    async fn close(&self, ctx: &Context<Msg>) {
        println!("close {:?} ", ctx.addr());
        
    }
}

async fn proxy_open(msg:Msg,ctx: Context<Msg>){
  let bytes=msg.data;
  let mut bf=ByteBuf::new_from(&bytes[..]);
  // let i1=bf.read_u8();
  // let i2=bf.read_u8();
  // let i3=bf.read_u8();
  // let i4=bf.read_u8();
  let host=bf.read_string_with_u8_be_len();
  let port = bf.read_u16_be();
  let addr=format!("{}:{}",host,port).to_string();
  let id=bf.read_u64_be();
  println!("fid->{},ip->{}",id,addr);
  let stream=TcpStream::connect(addr).await.unwrap();
  let addr=stream.peer_addr().unwrap();
  println!("代理连接{}",addr);
  Bootstrap::build(stream, addr, ProxyEncoder{}, ProxyDecoder{}, ProxyHandler{ctx: ctx.clone(),id:id}).run().await.unwrap();
}

async fn proxy_forward(msg:Msg){
  let bytes=msg.data;
  let mut bf=ByteBuf::new_from(&bytes[..]);
  let id=bf.read_u64_be();
  let mut buf=vec![0u8;bf.readable_bytes()];
  bf.read_bytes(&mut buf);
  println!("forward len {}:{} proxyMap {:?}",bytes.len(),buf.len(),proxy::ProxyMap.lock().await);
  match proxy::ProxyMap.lock().await.get(&id){
    Some(ctx)=>{
      ctx.write(buf.to_vec()).await;
    },
    None=>{
      println!("无对应id连接{}",id);
    }
  };
}
