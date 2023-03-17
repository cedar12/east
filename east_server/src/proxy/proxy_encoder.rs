use east_core::encoder::Encoder;

use super::ProxyMsg;




pub struct ProxyEncoder{}

impl Encoder<ProxyMsg> for ProxyEncoder{
    fn encode(&self,ctx:&east_core::context::Context<ProxyMsg>,msg:ProxyMsg,byte_buf:&mut east_core::byte_buf::ByteBuf) {
      byte_buf.write_bytes(&msg.buf);
    }
}