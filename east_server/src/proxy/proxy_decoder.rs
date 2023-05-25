use std::sync::Arc;

use east_core::{decoder::Decoder, byte_buf::ByteBuf};
use tokio::sync::Mutex;

use super::{ProxyMsg, speed::Tachometer};


pub struct ProxyDecoder{
  speed:Tachometer
}

impl ProxyDecoder{
  pub fn new()->Self{
    Self { speed:Tachometer::new() }
  }
}

#[async_trait::async_trait]
impl Decoder<ProxyMsg> for ProxyDecoder{
    async fn decode(&mut self,ctx: &east_core::context::Context<ProxyMsg> ,byte_buf: &mut ByteBuf) {
      if byte_buf.readable_bytes()==0{
        return
      }
      if self.speed.has(byte_buf.readable_bytes()){
        let speed:f64=self.speed.speed() as f64/1024f64;
        // log::info!("speed->{:.2}kb/s",speed);
      }
      let mut buf=vec![0u8;byte_buf.readable_bytes()];
      byte_buf.read_bytes(&mut buf);
      byte_buf.clean();
      ctx.out(ProxyMsg{buf:buf}).await;
    }
    
}