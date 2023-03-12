

use crate::{handler::Handler, context::Context, types::TypesEnum};

use super::Msg;



pub struct MsgHandler {}

#[async_trait::async_trait]
impl Handler<Msg> for MsgHandler {
    
    async fn read(&self, ctx: &Context<Msg>, msg: Msg) {
        
        // let msg=&msg as &dyn Any;
        println!("handle read {:?}", msg);
        // let msg_ack = Msg::new(msg.msg_type, msg.data);
        // println!("handle ack {:?}", msg_ack);
        let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
        ctx.write(m).await;
    }
    async fn active(&self, ctx: &Context<Msg>) {
        
        println!("active {:?}连接", ctx.addr());
        
    }
}
