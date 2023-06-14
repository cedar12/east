use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}, collections::HashMap};

use east_core::{handler::Handler, message::Msg, context::Context, types::TypesEnum, byte_buf::ByteBuf, bootstrap2::Bootstrap, token_bucket::TokenBucket, handler2::HandlerMut};
use async_trait::async_trait;
use east_plugin::agent::Agent;
use tokio::{net::TcpStream, spawn, sync::Mutex};

use crate::{connection, proxy::{Proxy, self, ProxyMsg, proxy_encoder::ProxyEncoder, proxy_decoder::ProxyDecoder, proxy_handler::ProxyHandler}, config, plugin, key};

pub mod msg_decoder;

pub const TIME_KEY:&str="heartbeat_time";

pub const CONN_TIME_KEY:&str="conn_time";

pub struct ServerHandler{
}

impl ServerHandler {
    pub fn new()->Self{
      ServerHandler { }
    }
}

#[async_trait]
impl Handler<Msg> for ServerHandler{
  async fn active(&mut self,ctx:&Context<Msg>){
    log::info!("{} try to connect",ctx.addr());
    match SystemTime::now().duration_since(UNIX_EPOCH) {
      Ok(n) => {
        ctx.set_attribute(CONN_TIME_KEY.into(), Box::new(n.as_secs())).await;
      },
      Err(e) => log::error!("{:?}",e),
    }
  }
  async fn read(&mut self,ctx:&Context<Msg>,msg:Msg){

    match msg.msg_type{
      TypesEnum::Auth=>{
        // let s=String::from_utf8(msg.data).unwrap();
        let s=key::decrypt(msg.data);
        match s{
          Ok(s)=>{
              
            log::info!("{} authentication",s);
            match agent_adapter(s.clone()).await{
              Some(agent)=>{
                let id=s.clone();
                let id2=s.clone();
                let id3=s.clone();
                let opt=connection::Conns.get(s).await;
                match opt{
                  Some(c)=>{
                    log::info!("{:?} already connected, can't connect again",c.id());
                    ctx.close().await;
                    return
                  }
                  None=>{
                    ctx.set_attribute("id".into(), Box::new(id2)).await;
                    let conn=connection::Connection::new(ctx.clone(),id);
                    connection::Conns.insert(id3.clone(),conn).await;
                    let msg=Msg::new(TypesEnum::Auth,vec![]);
                    ctx.write(msg).await;
                    for a in agent.proxy.iter(){
                      if !a.enable{
                        continue;
                      }
                      let bind_port=a.bind_port.clone();
                      let c=ctx.clone();
                      let id=id3.clone();
                      ctx.set_attribute("id".into(), Box::new(id)).await;
                      let id=id3.clone();
                      spawn(async move{
                          if let Some(conn)=connection::Conns.get(id.clone()).await{
                          let mut proxy=Proxy::new(bind_port);
                          conn.insert(bind_port,proxy.clone()).await;
                          if let Err(e)=proxy.listen().await{
                            log::error!("{:?}",e);
                            return
                          }
                          if let Err(e)=proxy.accept(id,c.clone()).await{
                            log::error!("{:?}",e);
                          }
                        }
                        
                      });
                    }
                    
                  }
                }
                
              },
              None=>{
                log::warn!("{} certification failed",s);
                ctx.close().await;
              }
            }

          },
          Err(e)=>{
            log::error!("key decryption error {:?}",e);
            ctx.close().await;
          }
        }
        
      },
      TypesEnum::ProxyOpen=>{
        
        let mut bf=ByteBuf::new_from(&msg.data);
        let fid=bf.read_u64_be();
        let stream=ctx.get_attribute(format!("{}_{}",proxy::STREAM,fid)).await;
        let stream=stream.lock().await;
        if let Some(boot) = stream.downcast_ref::<Arc<Mutex<Bootstrap<ProxyEncoder,ProxyDecoder,ProxyHandler,ProxyMsg,TcpStream>>>>() {

          let boot=Arc::clone(boot);
          ctx.remove_attribute(format!("{}_{}",proxy::STREAM,fid)).await;
          spawn(async move{
            let ret=boot.lock().await.run().await;
            if let Err(e)=ret{
              log::error!("{:?}",e);
            }
            log::info!("id->{} Closed",fid);
          });
          
        } else {
          log::warn!("{} None",fid);
        }
        
        
      },
      TypesEnum::ProxyForward=>{
        let mut bf=ByteBuf::new_from(&msg.data);
        let id=bf.read_u64_be();
        match proxy::ProxyMap.lock().await.get(&id){
          Some(ctx)=>{
            let mut buf=vec![0u8;bf.readable_bytes()];
            bf.read_bytes(&mut buf);
            ctx.write(ProxyMsg{buf:buf}).await;
          },
          None=>{
            log::warn!("{} No connection",id);
            let mut bf=ByteBuf::new_with_capacity(0);
            bf.write_u64_be(id).unwrap();
            let msg=Msg::new(TypesEnum::ProxyClose,bf.available_bytes().to_vec());
            ctx.write(msg).await;
          }
        }
      },
      TypesEnum::ProxyClose=>{
        let mut bf=ByteBuf::new_from(&msg.data);
        let id=bf.read_u64_be();
        
        let map=proxy::ProxyMap.lock().await;
        let result=map.get(&id);
        match result{
          Some(ctx)=>{
            ctx.close().await;
            log::info!("Close connection {} ",id);
          },
          None=>{
            log::warn!("No connection {}",id)
          }
        }
        
      },
      TypesEnum::Heartbeat=>{
        match SystemTime::now().duration_since(UNIX_EPOCH) {
          Ok(n) => {
            // log::info!("heartbeat->{}",n.as_secs());
            ctx.set_attribute(TIME_KEY.into(), Box::new(n.as_secs())).await;
          },
          Err(e) => log::error!("{:?}",e),
        }
      }
      TypesEnum::FileInfoAsk=>{
        if msg.data.len()>0{
          let err_msg=String::from_utf8(msg.data).unwrap();
          log::error!("发送文件信息错误：{}",err_msg);
        }else{
          let id_attr=ctx.get_attribute("id".into()).await;
          let id=id_attr.lock().await;
          if let Some(id)=id.downcast_ref::<String>(){
            match connection::Conns.get(id.clone()).await{
              Some(c)=>{
                log::info!("{}->准备发送文件",id);
                let path_attr=ctx.get_attribute("send_file_path".into()).await;
                let path=path_attr.lock().await;
                if let Some(path)=path.downcast_ref::<String>(){
                  let sender=c.file_sender_map.get(path);
                  if let Some(s)=sender{
                    s.send(()).await.unwrap();
                    ctx.remove_attribute("send_file_path".into()).await;
                  };
                  
                }else{
                  log::warn!("未获取到发送文件路径")
                }
                
              }
              None=>{
                log::warn!("{}连接未获取到代理绑定端口",id)
              }
            }
          }

        }
      },
      TypesEnum::FileAsk=>{
        if msg.data.len()>0{
          // 终止传输
          let err_msg=String::from_utf8(msg.data).unwrap();
          log::error!("发送文件错误：{}",err_msg);
        }
      }
      _=>{

      }
    }
  }

  async fn close(&mut self,ctx:&Context<Msg>){
    log::info!("{:?} disconnect",ctx.addr());
    let id_attr=ctx.get_attribute("id".into()).await;
    let id=id_attr.lock().await;
    if let Some(id)=id.downcast_ref::<String>(){
      
      proxy::remove(id).await;
      match connection::Conns.get(id.clone()).await{
        Some(c)=>{
          c.remove_all().await;
        }
        None=>{
          log::warn!("{} the connection did not get to the proxy binding port",id)
        }
      }
      connection::Conns.remove(id.clone()).await;
      log::info!("{} related connection removed",id);
    }
    ctx.remove_attribute("id".into()).await;
  }
}


async fn agent_adapter(id:String)->Option<Agent>{
  use east_plugin::proxy::Proxy;
  let plugin_result=plugin::database_plugin().await;
  match plugin_result{
    Ok((plugin,_pi))=>{
      // log::info!("使用插件{:?}",pi);
      let agent=plugin.get_agent(id.clone());
      match agent{
        Ok(agent)=>{
          // log::info!("{:?}",agent);
          return Some(agent)
        },
        Err(e)=>{
          log::error!("{}",e);
          return None
        }
      }
    },
    Err(_)=>{
      match config::CONF.agent.get(&id.clone()){
        Some(agents)=>{
          // log::info!("{:?}",agents);
          return Some(
            Agent{
              id:id.clone(),
              name: id.clone(),
              proxy: agents.iter().map(move |a|Proxy{
                bind_port: a.bind_port,
                target_host: a.target_host.clone(),
                target_port: a.target_port,
                enable: true,
                whitelist: a.whitelist.clone(),
                max_rate: a.max_rate,
              }).collect(),
            })
        },
        None=>{
          return None
        }
      }
    }
  }

}

