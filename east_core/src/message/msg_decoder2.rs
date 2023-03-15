use crate::decoder2::Decoder;
use crate::types::TypesEnum;
use super::Msg;
use crate::byte_buf::ByteBuf;
use crate::context2::Context;

pub struct MsgDecoder{}

impl Decoder<Msg> for MsgDecoder{
    
    fn decode(&self,ctx:&Context<Msg>,bf:&mut ByteBuf) {
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
        bf.clean();
        let msg=Msg::new( msg_type, buf);
        ctx.out(msg);
        self.decode(ctx, bf);
        
    }
    
}