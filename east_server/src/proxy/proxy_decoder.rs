use east_core::{decoder::Decoder, byte_buf::ByteBuf};

use super::ProxyMsg;



pub struct ProxyDecoder{}

#[async_trait::async_trait]
impl Decoder<ProxyMsg> for ProxyDecoder{
    async fn decode(&self,ctx: &east_core::context::Context<ProxyMsg> ,byte_buf: &mut ByteBuf) {
      if byte_buf.readable_bytes()==0{
        return
      }
      let mut buf=vec![0u8;byte_buf.readable_bytes()];
      byte_buf.read_bytes(&mut buf);
      ctx.out(ProxyMsg{buf:buf}).await;
    }
    
}