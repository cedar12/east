#[macro_use]
extern crate lazy_static; 
extern crate east_core;

mod handler;
mod proxy;

use east_core::{handler::Handler, message::{Msg, msg_encoder::MsgEncoder, msg_decoder::MsgDecoder}, context::Context, bootstrap::Bootstrap};
use handler::AgentHandler;
use tokio::{io, net::TcpStream};


#[tokio::main]
async fn main() ->io::Result<()>{
    let stream=TcpStream::connect("127.0.0.1:3555").await?;
    let addr=stream.peer_addr().unwrap();
    Bootstrap::build(stream, addr, MsgEncoder{}, MsgDecoder{}, AgentHandler{}).run().await?;
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
    async fn close(&self,ctx:&Context<Msg>){
        println!("{:?} 断开",ctx.addr())
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

    #[tokio::test]
    async fn test_s(){
        let s=String::from("你好!");
        println!("{:?}",s.as_bytes());
    }
}