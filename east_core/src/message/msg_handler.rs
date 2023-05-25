

use crate::{handler::Handler, context::Context, types::TypesEnum};

use super::Msg;



pub struct MsgHandler {}

#[async_trait::async_trait]
impl Handler<Msg> for MsgHandler {
    
    async fn read(&mut self, ctx: &Context<Msg>, msg: Msg) {
        println!("handle read {:?}", msg);
        let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
        ctx.write(m).await;
    }
    async fn active(&mut self, ctx: &Context<Msg>) {
        
        println!("active {:?}连接", ctx.addr());
        
    }
    async fn close(&mut self,ctx:&Context<Msg>){
        println!("close {:?}断开 ", ctx.addr());
    }
}
