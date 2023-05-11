use east_core::{decoder::Decoder, byte_buf::ByteBuf};



pub struct ProxyDecoder{}

#[async_trait::async_trait]
impl Decoder<Vec<u8>> for ProxyDecoder{
    async fn decode(&mut self,ctx: &east_core::context::Context<Vec<u8>> ,byte_buf: &mut ByteBuf) {
      if byte_buf.readable_bytes()==0{
        return
      }
      let mut buf=vec![0u8;byte_buf.readable_bytes()];
      byte_buf.read_bytes(&mut buf);
      byte_buf.clean();
      ctx.out(buf).await;
    }
    
}