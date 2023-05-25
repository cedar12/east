use crate::context::Context;
use async_trait::async_trait;

#[async_trait]
pub trait HandlerMut<T>{
  async fn active(&mut self,ctx:&Context<T>);
  async fn read(&mut self,ctx:&Context<T>,msg:T);
  async fn close(&mut self,ctx:&Context<T>);
}


#[async_trait]
pub trait Handler<T>{
  async fn active(&mut self,ctx:&mut crate::context2::Context<T>);
  async fn read(&mut self,ctx:&mut crate::context2::Context<T>,msg:T);
  async fn close(&mut self,ctx:&mut crate::context2::Context<T>);
}