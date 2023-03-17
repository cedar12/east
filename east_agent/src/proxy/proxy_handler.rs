use east_core::{handler::Handler, context::Context, message::Msg, types::TypesEnum, byte_buf::ByteBuf};

use crate::proxy;



pub struct ProxyHandler{
  pub ctx:Context<Msg>,
  pub id:u64,
}

#[async_trait::async_trait]
impl Handler<Vec<u8>> for ProxyHandler{
  async fn read(&self, ctx: &Context<Vec<u8>>, msg: Vec<u8>) {
    println!("proxy read len {:?}", msg.len());
    let mut bf=ByteBuf::new_with_capacity(0);
    bf.write_u64_be(self.id);
    bf.write_bytes(&msg);
    let m=Msg::new(TypesEnum::ProxyForward, bf.available_bytes().to_vec());
    self.ctx.write(m).await;
  }
  async fn active(&self, ctx: &Context<Vec<u8>>) {
    println!("proxy active {:?}", ctx.addr());
    proxy::ProxyMap.lock().await.insert(self.id,ctx.clone());
    let mut bf=ByteBuf::new_with_capacity(0);
    bf.write_u64_be(self.id);
    let msg=Msg::new(TypesEnum::ProxyOpen, bf.available_bytes().to_vec());
    self.ctx.write(msg).await;
    println!("回复代理打开成功")
  }
  async fn close(&self,ctx:&Context<Vec<u8>>){
    println!("close {:?} {}", ctx.addr(),self.id);
    proxy::ProxyMap.lock().await.remove(&self.id);
    println!("ProxyMap {:?} ", proxy::ProxyMap.lock().await);
    let mut bf=ByteBuf::new_with_capacity(0);
    bf.write_u64_be(self.id);
    let msg=Msg::new(TypesEnum::ProxyClose, bf.available_bytes().to_vec());
    self.ctx.write(msg).await;
  }
}