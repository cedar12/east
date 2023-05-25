use east_core::encoder::Encoder;
use std::{sync::{Arc, Mutex}};
use super::speed::Tachometer;
use super::ProxyMsg;

pub struct ProxyEncoder{
  speed:Tachometer
}

impl ProxyEncoder{
  pub fn new()->Self{
    Self { speed:Tachometer::new() }
  }

  
}

impl Encoder<ProxyMsg> for ProxyEncoder{
    fn encode(&mut self,ctx:&east_core::context::Context<ProxyMsg>,msg:ProxyMsg,byte_buf:&mut east_core::byte_buf::ByteBuf) {
      if self.speed.has(msg.buf.len()){
        let speed:f64=self.speed.speed() as f64/1024f64;
        // log::info!("speed->{:.2}kb/s",speed);
      }
      byte_buf.write_bytes(&msg.buf);
    }
}