use tokio::io::AsyncWriteExt;

use crate::encoder::Encoder;
use crate::byte_buf::ByteBuf;
use crate::context::Context;
use super::Msg;


pub struct MsgEncoder{}

impl Encoder<Msg> for MsgEncoder{
    fn encode(&self,ctx:&Context<Msg>,msg:Msg,byte_buf:&mut ByteBuf) {
        byte_buf.write_u8_be(0x86);
        byte_buf.write_u8_be(msg.msg_type as u8);
        byte_buf.write_u32_be(msg.data_len);
        byte_buf.write_bytes(&msg.data);
    }
}