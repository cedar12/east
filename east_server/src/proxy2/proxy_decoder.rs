use std::sync::Arc;

use east_core::{decoder2::Decoder, byte_buf::ByteBuf};
use tokio::sync::Mutex;

use super::{ProxyMsg, speed::Tachometer};


pub struct ProxyDecoder{
  speed:Arc<Mutex<Tachometer>>
}

impl ProxyDecoder{
  pub fn new()->Self{
    Self { speed:Arc::new(Mutex::new(Tachometer::new())) }
  }
}

#[async_trait::async_trait]
impl Decoder<ProxyMsg> for ProxyDecoder{
    async fn decode(&mut self,ctx: &mut east_core::context2::Context<ProxyMsg> ,byte_buf: &mut ByteBuf) {
      if byte_buf.readable_bytes()==0{
        return
      }
      let mut s=self.speed.lock().await;
      if s.has(byte_buf.readable_bytes()){
        let speed:f64=s.speed() as f64/1024f64;
        // log::info!("speed->{:.2}kb/s",speed);
      }
      let mut buf=vec![0u8;byte_buf.readable_bytes()];
      byte_buf.read_bytes(&mut buf);
      byte_buf.clean();
      ctx.out(ProxyMsg{buf:buf}).await;
    }
    
}