use crate::encoder::Encoder;
use crate::byte_buf::ByteBuf;
use crate::context::Context;
use crate::encoder2::EncoderMut;
use super::Msg;


pub struct MsgEncoder{}

impl Encoder<Msg> for MsgEncoder{
    fn encode(&mut self,ctx:&Context<Msg>,msg:Msg,byte_buf:&mut ByteBuf) {
        byte_buf.write_u8_be(0x86);
        byte_buf.write_u8_be(msg.msg_type as u8);
        byte_buf.write_u32_be(msg.data_len);
        byte_buf.write_bytes(&msg.data);
    }
}

pub struct MsgMutEncoder{}
#[async_trait::async_trait]
impl EncoderMut<Msg> for MsgMutEncoder{
    async fn encode(&mut self,ctx:&Context<Msg>,msg:Msg,byte_buf:&mut ByteBuf) {
        byte_buf.write_u8_be(0x86);
        byte_buf.write_u8_be(msg.msg_type as u8);
        byte_buf.write_u32_be(msg.data_len);
        byte_buf.write_bytes(&msg.data);
    }
}