use std::time::{SystemTime, UNIX_EPOCH};

use east_core::decoder::Decoder;
use east_core::types::TypesEnum;
use east_core::message::{Msg,MSG_HEADER_LEN};
use east_core::byte_buf::ByteBuf;
use east_core::context::Context;

const AUTH_TIMEOUT:u64=3;

pub struct MsgDecoder{}

#[async_trait::async_trait]
impl Decoder<Msg> for MsgDecoder{
    
    async fn decode(&mut self,ctx:&Context<Msg>,bf:&mut ByteBuf) {
        let time=ctx.get_attribute(super::CONN_TIME_KEY.into()).await;
        let t=time.lock().await;
        if let Some(t)=t.downcast_ref::<u64>(){
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => {
                    if n.as_secs()-t>AUTH_TIMEOUT{
                        log::warn!("超过{}秒未认证，关闭连接",AUTH_TIMEOUT);
                        ctx.close().await;
                    }
                },
                Err(e) => log::error!("{:?}",e),
            }
            
        }
        
        if bf.readable_bytes()<MSG_HEADER_LEN{
            return
        }
        bf.mark_reader_index();
        let flag=bf.read_u8();
        if flag!=0x86{
            return
        }
        let t=bf.read_u8();

        let msg_type=TypesEnum::try_from(t).unwrap();
                
        let len=bf.read_u32_be() as usize;

        if bf.readable_bytes()<len{
            bf.reset_reader_index();
            return
        }
        let mut buf=vec![0u8;len];
        bf.read_bytes(&mut buf);
        bf.clean();

        let msg=Msg::new( msg_type, buf);
        
        ctx.out(msg).await;
        self.decode(ctx, bf).await;
        
    }
    
}