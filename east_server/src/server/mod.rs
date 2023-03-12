use std::net::SocketAddr;

use east_core::bootstrap::Bootstrap;
use east_core::message::msg_decoder::MsgDecoder;
use east_core::message::msg_encoder::MsgEncoder;
use east_core::message::msg_handler::MsgHandler;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Result,AsyncReadExt,AsyncWriteExt};
use east_core::byte_buf::ByteBuf;
use log;
use east_core::types::TypesEnum;
use east_core::message::Msg;


pub async fn run() -> Result<()> {
    
    let listener=TcpListener::bind("0.0.0.0:3555").await.unwrap();
    loop{
        let (socket,addr)=listener.accept().await.unwrap();
        // log::info!("{}连接上",addr);
        tokio::spawn(process_socket(socket,addr));
    }
    Ok(())
}

async fn process_socket(mut client:TcpStream,addr:SocketAddr)->Result<()>{
    // let c=Connection::new(client,"".to_string());
    // let mut sockets=SOCKETS.lock().await;
    // sockets.push(c);
    // let mut bf=ByteBuf::new_with_capacity(1024);
    
    // let mut buf=vec![0u8;1024];
    // loop {
    //     let n=client.read(&mut buf).await?;
    //     if n==0{
    //         println!("{} 连接关闭",client.peer_addr()?);
    //         return Ok(())
    //     }
    //     bf.write_bytes(&buf[..n])?;
    //     encode(&mut bf);
    // }
    Bootstrap::build(client,addr, MsgEncoder{}, MsgDecoder{}, MsgHandler{}).run().await?;
    Ok(())
}

const HEADER_LEN:usize=6;

// fn encode(bf:&mut ByteBuf){
//     if bf.readable_bytes()<HEADER_LEN{
//         return
//     }
//     bf.mark_reader_index();
//     let flag=bf.read_u8();
//     if flag!=0x86{
//         return
//     }
//     let t=bf.read_u8();

//     match TypesEnum::try_from(t) {
//         Err(())=>return,
//         Ok(msg_type)=>{
            
//             let len=bf.read_u32_be() as usize;

//             if bf.readable_bytes()<len{
//                 bf.reset_reader_index();
//                 return
//             }
//             let mut buf=vec![0u8;len];
//             bf.read_bytes(&mut buf);

//             let msg=Message::new(flag, msg_type, buf);
//             // 包
//             log::info!("{:?}",msg);
//             println!("{:?}",msg);
//             encode(bf)
//         }
//     }
    
// }