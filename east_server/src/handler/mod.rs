use std::sync::Arc;

use east_core::{handler::Handler, message::Msg, context::Context, types::TypesEnum, byte_buf::ByteBuf, bootstrap::Bootstrap};
use async_trait::async_trait;
use anyhow::Result;
use tokio::{net::TcpStream, spawn, sync::Mutex};

use crate::{connection, proxy::{Proxy, self, ProxyMsg, proxy_encoder::ProxyEncoder, proxy_decoder::ProxyDecoder, proxy_handler::ProxyHandler}};
pub struct ServerHandler{
}

#[async_trait]
impl Handler<Msg> for ServerHandler{
  async fn active(&self,ctx:&Context<Msg>){
    println!("{} 已连接上",ctx.addr());
  }
  async fn read(&self,ctx:&Context<Msg>,msg:Msg){
    println!("server read {:?} {:?}",msg.msg_type,msg.data.len());
    // match handle(ctx,msg).await{
    //   Ok(())=>{},
    //   Err(e)=>{
    //     println!("error {:?}",e);
    //   }
    // };
    match msg.msg_type{
      TypesEnum::Auth=>{
        let s=String::from_utf8(msg.data).unwrap();
        println!("认证 {}",s);
        let id=s.clone();
        let id2=s.clone();
        let id3=s.clone();
        ctx.set_attribute("id".into(), Box::new(id2)).await;
        let opt=connection::Conns.get(s).await;
        match opt{
          Some(c)=>{
            println!("{:?} 已经连接了，不能重复连接",c);
          }
          None=>{
            let conn=connection::Connection::new(ctx.clone(),id);
            connection::Conns.push(conn).await;
            
            let c=ctx.clone();
            spawn(async move{
              let mut proxy=Proxy::new("0.0.0.0:8089".into());
              proxy.listen().await.unwrap();
              proxy.accept(id3,c.clone()).await.unwrap();
            });
            
            // ctx.set_attribute("proxy".into(), Box::new(proxy)).await;
          }
        }
        
       
      },
      TypesEnum::ProxyOpen=>{
        println!("打开转发");
        
        let mut bf=ByteBuf::new_from(&msg.data);
        let fid=bf.read_u64_be();
        // let id=ctx.get_attribute("id".into()).await;
        let stream=ctx.get_attribute(format!("{}_{}",proxy::STREAM,fid)).await;
        // let proxy=ctx.get_attribute("proxy".into()).await;
        let stream=stream.lock().await;
        // let mut proxy=proxy.lock().await;
        if let Some(boot) = stream.downcast_ref::<Arc<Mutex<Bootstrap<ProxyEncoder,ProxyDecoder,ProxyHandler,ProxyMsg>>>>() {
          // println!("id->{:?}", id);
          let boot=Arc::clone(boot);
          ctx.remove_attribute(format!("{}_{}",proxy::STREAM,fid)).await;
          spawn(async move{
            boot.lock().await.run().await.unwrap();
            println!("id->{},已关闭",fid);
          });
          
          // match proxy.downcast_mut::<Proxy>(){
          //   Some(proxy)=>{
          //     proxy.accpet(id.clone(),ctx.clone()).await.unwrap();
          //   },
          //   None=>{
          //     println!("接收代理连接错误");
          //   }
          // }
        } else {
          println!("无boot->{}",fid);
        }
        
        
      },
      TypesEnum::ProxyForward=>{
        // println!("转发数据 {:?}",proxy::ProxyMap.lock().await);
        let mut bf=ByteBuf::new_from(&msg.data);
        let id=bf.read_u64_be();
        match proxy::ProxyMap.lock().await.get(&id){
          Some(ctx)=>{
            let mut buf=vec![0u8;bf.readable_bytes()];
            bf.read_bytes(&mut buf);
            ctx.write(ProxyMsg{buf:buf}).await;
          },
          None=>{
            println!("无代理连接 {}",id)
          }
        }
      },
      TypesEnum::ProxyClose=>{
        let mut bf=ByteBuf::new_from(&msg.data);
        let id=bf.read_u64_be();
         
        
        let map=proxy::ProxyMap.lock().await;
        println!("server close {} {:?} ",id, map);
        let result=map.get(&id);
        match result{
          Some(ctx)=>{
            println!("开始close {:?}",ctx);
            ctx.close().await;
            // proxy::ProxyMap.lock().await.remove(&id);
            println!("server ProxyMap close {} {:?} ",id, map);
          },
          None=>{
            println!("无代理连接 {} {:?}",id,proxy::ProxyMap.lock().await)
          }
        }
        
      }
    }
    // connection::Conns.println().await;
    // let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
    // ctx.write(m).await;
  }
  async fn close(&self,ctx:&Context<Msg>){
    println!("{:?} 断开",ctx.addr());
    connection::Conns.remove(ctx.clone()).await;
  }
}

// async fn handle(ctx:&Context<Msg>,msg:Msg)->Result<()>{
  
//   Ok(())
// }