

use crate::{handler::Handler, context::Context, types::TypesEnum};

use super::Msg;



pub struct MsgHandler {}

#[async_trait::async_trait]
impl Handler<Msg> for MsgHandler {
    
    async fn read(&self, ctx: &Context<Msg>, msg: Msg) {
        println!("handle read {:?}", msg);
        let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
        ctx.write(m).await;
    }
    async fn active(&self, ctx: &Context<Msg>) {
        
        println!("active {:?}连接", ctx.addr());
        
    }
    async fn close(&self,ctx:&Context<Msg>){
        println!("close {:?}断开 ", ctx.addr());
    }
}
