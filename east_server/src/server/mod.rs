use std::net::SocketAddr;

use east_core::bootstrap::Bootstrap;
use east_core::message::msg_decoder::MsgDecoder;
use east_core::message::msg_encoder::MsgEncoder;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Result};
use crate::handler::ServerHandler;

const BIND_ADDR:&'static str="0.0.0.0:3555";

pub async fn run() -> Result<()> {
    let listener=TcpListener::bind(BIND_ADDR).await.unwrap();
    println!("服务启动->{}",BIND_ADDR);
    loop{
        let (socket,addr)=listener.accept().await.unwrap();
        // log::info!("{}连接上",addr);
        tokio::spawn(async move{
            process_socket(socket,addr).await.unwrap();
        });
    }
}

async fn process_socket(client:TcpStream,addr:SocketAddr)->Result<()>{
    Bootstrap::build(client,addr, MsgEncoder{}, MsgDecoder{}, ServerHandler{}).run().await.unwrap();
    Ok(())
}