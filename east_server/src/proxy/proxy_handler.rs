use east_core::{handler::Handler, context::Context, message::Msg, types::TypesEnum, byte_buf::ByteBuf, handler2::HandlerMut};

use crate::proxy;

use super::ProxyMsg;


const PORT_KEY:&str="port";


pub struct ProxyHandler{
  pub ctx:Context<Msg>,
  pub id:u64,
  pub conn_id:String,
  pub port:u16,
}

#[async_trait::async_trait]
impl Handler<ProxyMsg> for ProxyHandler{
  async fn read(&self, ctx: &Context<ProxyMsg>, msg: ProxyMsg) {
    // println!("proxy read len {:?}", msg.buf.len());
    // let m=Msg::new(TypesEnum::ProxyForward, msg);
    // self.ctx.write(m).await;
    let mut bytes=msg.buf;
    let mut id_bytes=self.id.to_be_bytes().to_vec();
    id_bytes.append(&mut bytes);
    let msg=Msg::new(TypesEnum::ProxyForward,id_bytes);
    self.ctx.write(msg).await;

  }
  async fn active(&self, ctx: &Context<ProxyMsg>) {
    log::info!("[{}]proxy active {:?} id->{}",self.conn_id, ctx.addr(),self.id);
    ctx.set_attribute(PORT_KEY.into(),Box::new(self.port)).await;
    proxy::ProxyMap.lock().await.insert(self.id,ctx.clone());
    let mut id_map=proxy::IdMap.lock().await;
    match id_map.get_mut(&self.conn_id){
      Some(v)=>{
        v.push(self.id);
      },
      None=>{
        id_map.insert(self.conn_id.clone(), vec![self.id]);
      }
    }
  }
  async fn close(&self,ctx:&Context<ProxyMsg>){
    let mut map=proxy::ProxyMap.lock().await;
    log::info!("proxy active close {:?}  id->{}", ctx.addr(),self.id);
    map.remove(&self.id);
    let mut id_map=proxy::IdMap.lock().await;
    match id_map.get_mut(&self.conn_id.clone()){
      Some(v)=>{
        let some_x = self.id;
        v.retain(|&x| x != some_x);
      },
      None=>{
        log::warn!("无连接id->{}",self.conn_id);
      }
    }
    // self.ctx.remove_attribute(format!("{}_{}",proxy::STREAM,self.id)).await;
    
  }
}



#[async_trait::async_trait]
impl HandlerMut<ProxyMsg> for ProxyHandler{
  async fn read(&mut self, ctx: &Context<ProxyMsg>, msg: ProxyMsg) {
    
    let mut bytes=msg.buf;
    let mut id_bytes=self.id.to_be_bytes().to_vec();
    id_bytes.append(&mut bytes);
    let msg=Msg::new(TypesEnum::ProxyForward,id_bytes);
    self.ctx.write(msg).await;

  }
  async fn active(&mut self, ctx: &Context<ProxyMsg>) {
    log::info!("[{}]proxy active {:?} id->{}",self.conn_id, ctx.addr(),self.id);
    ctx.set_attribute(PORT_KEY.into(),Box::new(self.port)).await;
    proxy::ProxyMap.lock().await.insert(self.id,ctx.clone());
    let mut id_map=proxy::IdMap.lock().await;
    match id_map.get_mut(&self.conn_id){
      Some(v)=>{
        v.push(self.id);
      },
      None=>{
        id_map.insert(self.conn_id.clone(), vec![self.id]);
      }
    }
  }
  async fn close(&mut self,ctx:&Context<ProxyMsg>){
    let mut map=proxy::ProxyMap.lock().await;
    log::info!("proxy active close {:?}  id->{}", ctx.addr(),self.id);
    map.remove(&self.id);
    let mut id_map=proxy::IdMap.lock().await;
    match id_map.get_mut(&self.conn_id.clone()){
      Some(v)=>{
        let some_x = self.id;
        v.retain(|&x| x != some_x);
      },
      None=>{
        log::warn!("无连接id->{}",self.conn_id);
      }
    }
    
  }
}