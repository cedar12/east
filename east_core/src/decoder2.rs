use crate::byte_buf::ByteBuf;
use crate::context2::Context;

pub trait Decoder<T>{

  fn decode(&self,ctx:&Context<T>,byte_buf:&mut ByteBuf);
}