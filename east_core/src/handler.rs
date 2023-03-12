use crate::context::Context;

#[async_trait::async_trait]
pub trait Handler<T>{
  async fn active(&self,ctx:&Context<T>);
  async fn read(&self,ctx:&Context<T>,msg:T);
}