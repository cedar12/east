use crate::decoder::Decoder;
use crate::types::TypesEnum;
use super::Msg;
use crate::byte_buf::ByteBuf;
use crate::context::Context;

pub struct MsgDecoder{}

#[async_trait::async_trait]
impl Decoder<Msg> for MsgDecoder{
    
    async fn decode(&self,ctx:&Context<Msg>,bf:&mut ByteBuf) {
        if bf.readable_bytes()<super::MSG_HEADER_LEN{
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

        let msg=Msg::new( msg_type, buf);
        // 包
        // log::info!("{:?}",msg);
        println!("{:?}",msg);
        ctx.out(msg).await;
        self.decode(ctx, bf).await;
        
    }
    
}