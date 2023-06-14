#[macro_use]
extern crate lazy_static; 
extern crate east_core;

mod handler;
mod proxy;
mod config;
mod log_conf;
mod key;

#[cfg(test)]
mod tests;

use std::sync::Arc;

use east_core::{bootstrap2::Bootstrap, message::{msg_encoder::{MsgEncoder}, msg_decoder::{MsgDecoder}}};
use handler::{AgentHandler};
use tokio::{io, net::TcpStream, time};


#[tokio::main]
async fn main() ->io::Result<()>{
    log_conf::init();
    
    let conf=Arc::clone(&config::CONF);
    let version: &'static str = env!("CARGO_PKG_VERSION");
    log::info!("Version: {}", version);
    let author: &'static str = env!("CARGO_PKG_AUTHORS");
    log::info!("Author: {}", author);
    loop{
        let stream=TcpStream::connect(conf.server.clone()).await;
        match stream{
            Ok(stream)=>{
                let addr=stream.peer_addr().unwrap();
                let result=Bootstrap::build(stream, addr, MsgEncoder{}, MsgDecoder{}, AgentHandler::new()).run().await;
                if let Err(e)=result{
                    log::error!("{:}",e);
                }
            },
            Err(e)=>{
                log::error!("{:?}",e)
            }
        }
        log::info!("Waiting for reconnection");
        time::sleep(time::Duration::from_millis(3000)).await;
    }
}
