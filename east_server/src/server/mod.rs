use std::net::{SocketAddr};
use std::sync::Arc;

use east_core::bootstrap2::Bootstrap;
use east_core::message::msg_decoder::MsgDecoder;
use east_core::message::msg_encoder::MsgEncoder;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Result};
use crate::{config, connection};
use crate::handler::ServerHandler;

pub async fn run() -> Result<()> {
    let conf=Arc::clone(&config::CONF);
    let addr:SocketAddr=conf.server.bind.parse().unwrap();
    let listener=TcpListener::bind(addr).await.unwrap();
    log::info!("服务启动->{}",addr);
    connection::Conns.clear_invalid_connection().await;
    loop{
        let (socket,addr)=listener.accept().await.unwrap();
        tokio::spawn(async move{
            process_socket(socket,addr).await;
        });
    }
}

async fn process_socket(client:TcpStream,addr:SocketAddr){
    if let Err(e)=Bootstrap::build(client,addr, MsgEncoder{}, MsgDecoder{}, ServerHandler::new()).run().await{
        log::error!("{:?}",e);
    }
}
