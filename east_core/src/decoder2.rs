use crate::byte_buf::ByteBuf;
use crate::context::Context;
use async_trait::async_trait;

#[async_trait]
pub trait DecoderMut<T>{

  async fn decode(&mut self,ctx:&Context<T>,byte_buf:&mut ByteBuf);
}