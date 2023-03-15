use crate::byte_buf::ByteBuf;
use crate::context2::Context;

pub trait Encoder<T>{
  fn encode(&self,ctx:&Context<T>,msg:T,byte_buf:&mut ByteBuf);
}