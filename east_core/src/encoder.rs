use crate::byte_buf::ByteBuf;
use crate::context::Context;

pub trait Encoder<T>{
  fn encode(&mut self,ctx:&Context<T>,msg:T,byte_buf:&mut ByteBuf);
}