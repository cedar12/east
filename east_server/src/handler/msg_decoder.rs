use east_core::decoder::Decoder;
use east_core::types::TypesEnum;
use east_core::message::{Msg,MSG_HEADER_LEN};
use east_core::byte_buf::ByteBuf;
use east_core::context::Context;

pub struct MsgDecoder{}

#[async_trait::async_trait]
impl Decoder<Msg> for MsgDecoder{
    
    async fn decode(&self,ctx:&Context<Msg>,bf:&mut ByteBuf) {
        let id_attr=ctx.get_attribute("id".into()).await;
        let id=id_attr.lock().await;
        let mut is_auth=id.downcast_ref::<String>().is_some();
        
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
        let id_attr=ctx.get_attribute("id".into()).await;
        let id=id_attr.lock().await;
        let msg=Msg::new( msg_type, buf);
        if let None=id.downcast_ref::<String>(){
            if msg.msg_type!=TypesEnum::Auth&&msg.msg_type!=TypesEnum::Heartbeat{
                log::warn!("未认证，关闭连接");
                ctx.close().await;
            }
        }
        
        ctx.out(msg).await;
        self.decode(ctx, bf).await;
        
    }
    
}