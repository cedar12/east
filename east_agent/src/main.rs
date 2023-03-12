#[macro_use]
extern crate east_core;

use east_core::{handler::Handler, message::Msg, context::Context};
use tokio::io;


#[tokio::main]
async fn main() ->io::Result<()>{

    Ok(())
}

pub struct CMsgHandler {}

#[async_trait::async_trait]
impl Handler<Msg> for CMsgHandler {
    
    async fn read(&self, ctx: &Context<Msg>, msg: Msg) {
        
        // let msg=&msg as &dyn Any;
        println!("agent handle read {:?}", msg);
        // let msg_ack = Msg::new(msg.msg_type, msg.data);
        // println!("handle ack {:?}", msg_ack);
            // println!("active {:?}连接", ctx.peer_addr().await);
            // ctx.write(msg).await;
        
    }
    async fn active(&self, ctx: &Context<Msg>) {
        println!("active {:?}连接", ctx.addr());
        
    }
}

#[cfg(test)]
mod tests{
    use east_core::bootstrap::Bootstrap;
    use east_core::message::msg_decoder::MsgDecoder;
    use east_core::message::msg_encoder::MsgEncoder;
    use tokio::io::{Result,AsyncWriteExt};
    use tokio::net::{TcpStream};

    use east_core::types::TypesEnum;

    use crate::CMsgHandler;

    #[tokio::test]
    async fn test_client()->Result<()>{
        let mut stream=TcpStream::connect("127.0.0.1:3555").await?;
        let addr=stream.peer_addr()?;
        stream.write(&[0x86,TypesEnum::Auth as u8,0,0,0,2,0,1]).await?;
        Bootstrap::build(stream,addr, MsgEncoder{}, MsgDecoder{}, CMsgHandler{}).run().await?;
        Ok(())
    }
}