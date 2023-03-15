use std::sync::Arc;

use east_core::{handler::Handler, message::Msg, context::Context, types::TypesEnum};
use async_trait::async_trait;
use anyhow::Result;

use crate::connection;
pub struct ServerHandler{}

#[async_trait]
impl Handler<Msg> for ServerHandler{
  async fn active(&self,ctx:&Context<Msg>){
    println!("{} 已连接上",ctx.addr());
  }
  async fn read(&self,ctx:&Context<Msg>,msg:Msg){
    println!("server read {:?}",msg);
    match handle(ctx,msg).await{
      Ok(())=>{},
      Err(e)=>{
        println!("error {:?}",e);
      }
    };
    // let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
    // ctx.write(m).await;
  }
  async fn close(&self,ctx:&Context<Msg>){
    println!("{:?} 断开",ctx.addr());
    connection::Conns.remove(ctx.clone()).await;
  }
}

async fn handle(ctx:&Context<Msg>,msg:Msg)->Result<()>{
  match msg.msg_type{
    TypesEnum::Auth=>{
      
      let s=String::from_utf8(msg.data)?;
      println!("认证 {}",s);
      let id=s.clone();
      let id2=s.clone();
      ctx.set_attribute("id".into(), Box::new(id2)).await;
      let opt=connection::Conns.get(s).await;
      match opt{
        Some(c)=>{
          println!("{:?} 已经连接",c);
        }
        None=>{
          let conn=connection::Connection::new(ctx.clone(),id);
          connection::Conns.push(conn).await;
        }
      }
      
     
    },
    TypesEnum::ProxyOpen=>{
      println!("打开转发");
      let id=ctx.get_attribute("id".into()).await;
      let id=id.lock().await;
      if let Some(id) = id.downcast_ref::<String>() {
        println!("id->{:?}", id);
      } else {
          println!("Not a string...");
      }
    },
    TypesEnum::ProxyForward=>println!("转发数据"),
  }
  connection::Conns.println().await;
  Ok(())
}