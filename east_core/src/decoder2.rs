use crate::byte_buf::ByteBuf;
use crate::context::Context;
use async_trait::async_trait;

#[async_trait]
pub trait DecoderMut<T>{

  async fn decode(&mut self,ctx:&Context<T>,byte_buf:&mut ByteBuf);
}


#[async_trait]
pub trait Decoder<T>{

  async fn decode(&mut self,ctx:&mut crate::context2::Context<T>,byte_buf:&mut ByteBuf);
}