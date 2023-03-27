use std::thread;

use east_plugin::control::AgentControl;
use tokio::spawn;

use crate::connection;



pub struct AgentControlImpl{}

impl AgentControlImpl{
  pub fn new()->Self{
    AgentControlImpl {}
  }
}

impl AgentControl for AgentControlImpl{

    fn close(&self,agent_id:String) {
      let rt=tokio::runtime::Runtime::new().unwrap();
      let jh=thread::spawn(move ||{
        rt.block_on(async move{
          let id=agent_id.clone();
          let conn=connection::Conns.get(agent_id).await;
          if let Some(conn)=conn{
            log::info!("关闭代理端->{}",id);
            conn.ctx().close().await;
          }
        });
      });
      jh.join().unwrap();
      
    }
}