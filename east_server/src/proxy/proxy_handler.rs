use east_core::{handler::Handler, context::Context, message::Msg, types::TypesEnum, byte_buf::ByteBuf};

use crate::proxy;

use super::ProxyMsg;



pub struct ProxyHandler{
  pub ctx:Context<Msg>,
  pub id:u64,
}

#[async_trait::async_trait]
impl Handler<ProxyMsg> for ProxyHandler{
  async fn read(&self, ctx: &Context<ProxyMsg>, msg: ProxyMsg) {
    println!("proxy read len {:?}", msg.buf.len());
    // let m=Msg::new(TypesEnum::ProxyForward, msg);
    // self.ctx.write(m).await;
    let mut bytes=msg.buf;
    let mut id_bytes=self.id.to_be_bytes().to_vec();
    id_bytes.append(&mut bytes);
    let msg=Msg::new(TypesEnum::ProxyForward,id_bytes);
    self.ctx.write(msg).await;

  }
  async fn active(&self, ctx: &Context<ProxyMsg>) {
    println!("proxy active {:?} id->{}", ctx.addr(),self.id);
    proxy::ProxyMap.lock().await.insert(self.id,ctx.clone());
  }
  async fn close(&self,ctx:&Context<ProxyMsg>){
    let mut map=proxy::ProxyMap.lock().await;
    println!("proxy active close {:?}  id->{} {:?}", ctx.addr(),self.id,map);
    map.remove(&self.id);
    println!("ProxyMap {:?} ", map);
    if !ctx.is_run(){
      return;
    }
    // self.ctx.remove_attribute(format!("{}_{}",proxy::STREAM,self.id)).await;
    let mut bf=ByteBuf::new_with_capacity(0);
    bf.write_u64_be(self.id);
    let msg=Msg::new(TypesEnum::ProxyClose, bf.available_bytes().to_vec());
    self.ctx.write(msg).await;
  }
}