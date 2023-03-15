use crate::context::Context;

/// ```
/// #[async_trait::async_trait]
/// pub trait Handler<T>{
///   async fn active(&self,ctx:&Context<T>);
///   async fn read(&self,ctx:&Context<T>,msg:T);
///   async fn close(&self,ctx:&Context<T>);
/// }
/// 
/// pub struct MsgHandler {}
/// 
/// #[async_trait::async_trait]
/// impl Handler<Msg> for MsgHandler {
///     
///     async fn read(&self, ctx: &Context<Msg>, msg: Msg) {
///         println!("read {:?}", msg);
///         let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
///         ctx.write(m).await;
///     }
///     async fn active(&self, ctx: &Context<Msg>) {
///         
///         println!("active {:?}", ctx.addr());
///         
///     }
///     async fn close(&self,ctx:&Context<Msg>){
///         println!("close {:?} ", ctx.addr());
///     }
/// }
/// 
/// ```
/// 
#[async_trait::async_trait]
pub trait Handler<T>{
  async fn active(&self,ctx:&Context<T>);
  async fn read(&self,ctx:&Context<T>,msg:T);
  async fn close(&self,ctx:&Context<T>);
}