use east_core::{handler2::Handler, message::Msg, context2::Context, types::TypesEnum};
use anyhow::Result;

use crate::connection2 as connection;
pub struct ServerHandler{}

impl Handler<Msg> for ServerHandler{
  fn active(&self,ctx:&Context<Msg>){
    println!("{} 已连接上",ctx.addr());
  }
  fn read(&self,ctx:&Context<Msg>,msg:Msg){
    println!("server read {:?}",msg);
    match handle(ctx,msg){
      Ok(())=>{},
      Err(e)=>{
        println!("error {:?}",e);
      }
    };
    // let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
    // ctx.write(m).await;
  }
  fn close(&self,ctx:&Context<Msg>){
    println!("{:?} 断开",ctx.addr());
    connection::Conns.remove(ctx.clone());
  }
}

fn handle(ctx:&Context<Msg>,msg:Msg)->Result<()>{
  match msg.msg_type{
    TypesEnum::Auth=>{
      
      let s=String::from_utf8(msg.data)?;
      println!("认证 {}",s);
      let id=s.clone();
      let id2=s.clone();
      
      let opt=connection::Conns.get(s);
      match opt{
        Some(c)=>{
          println!("{:?} 已经连接",c);
        }
        None=>{
          let conn=connection::Connection::new(ctx.clone(),id);
          connection::Conns.push(conn);
        }
      }
      
     
    },
    TypesEnum::ProxyOpen=>{
      println!("打开转发");
     
    },
    TypesEnum::ProxyForward=>println!("转发数据"),
  }
  connection::Conns.println();
  Ok(())
}