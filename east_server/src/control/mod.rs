use std::{thread::{spawn}, sync::{Arc, Mutex}};

use east_plugin::control::{AgentControl, ProxyControl};

use crate::{connection, proxy::{Proxy, self}};



pub struct AgentControlImpl{}

impl AgentControlImpl{
  pub fn new()->Self{
    AgentControlImpl {}
  }
}

impl AgentControl for AgentControlImpl{

    fn is_online(&self,agent_id:String)->bool{
      let online=Arc::new(Mutex::new(false));
      let online_ret=Arc::clone(&online);
      let rt=tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async move{
        let conn=connection::Conns.get(agent_id).await;
        let mut online=online.lock().unwrap();
        if let Some(_)=conn{
          *online=true;
        }else{
          *online=false;
        }
      });
      
      let online=online_ret.lock().unwrap();
      online.clone()
    }

    fn close(&self,agent_id:String) {
      let rt=tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async move{
        let id=agent_id.clone();
        let conn=connection::Conns.get(agent_id).await;
        if let Some(conn)=conn{
          log::info!("关闭代理端->{}",id);
          conn.ctx().close().await;
        }
      });
      
    }

    fn send_file(&self,agent_id:String,path:String,target:String) {
      let rt=tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async move {
        let signal=connection::FileSignal.lock().await;
        if let Some(tx)=signal.as_ref(){
          tx.send((agent_id,path,target)).await.unwrap();
        }
      });
    }
}


pub struct ProxyControlImpl{}

impl ProxyControlImpl{
  pub fn new()->Self{
    ProxyControlImpl {}
  }
}

impl ProxyControl for ProxyControlImpl{
    fn start(&self,id:String,bind_port:u16) {
      let rt=tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async move {
        let signal=proxy::Signal.lock().await;
        if let Some(tx)=signal.as_ref(){
          tx.send((true,id,bind_port)).await.unwrap();
        }
        
        // if let Some(conn)=connection::Conns.get(id.clone()).await{
        //   let mut proxy=Proxy::new(bind_port);
        //   conn.insert(bind_port,proxy.clone()).await;
        //   if let Err(e)=proxy.listen().await{
        //     log::error!("{:?}",e);
        //     return
        //   }
        //   log::info!("开启代理转发端口->{}",bind_port);
        //   tokio::spawn(async move{
        //     if let Err(e)=proxy.accept(id,conn.ctx().clone()).await{
        //       log::error!("{:?}",e);
        //     }
        //   });
        // }
      });
    }

    fn stop(&self,id:String,bind_port:u16) {
      let rt=tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async move {
        // if let Some(conn)=connection::Conns.get(id.clone()).await{
        //   log::info!("关闭代理转发端口->{}",bind_port);
        //   conn.remove(bind_port).await;
        // }
        let signal=proxy::Signal.lock().await;
        if let Some(tx)=signal.as_ref(){
          tx.send((false,id,bind_port)).await.unwrap();
        }
        
      });
    }
}
