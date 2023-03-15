use crate::context2::Context;

pub trait Handler<T>{
  fn active(&self,ctx:&Context<T>);
  fn read(&self,ctx:&Context<T>,msg:T);
  fn close(&self,ctx:&Context<T>);
}