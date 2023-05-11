use crate::byte_buf::ByteBuf;
use crate::context::Context;
use async_trait::async_trait;

#[async_trait]
pub trait EncoderMut<T>{
  async fn encode(&mut self,ctx:&Context<T>,msg:T,byte_buf:&mut ByteBuf);
}



pub trait Encoder<T>{
  fn encode(&mut self,ctx:&mut crate::context2::Context<T>,msg:T,byte_buf:&mut ByteBuf);
}