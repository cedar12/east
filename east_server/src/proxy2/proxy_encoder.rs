use east_core::encoder2::Encoder;
use std::{sync::{Arc, Mutex}};
use super::speed::Tachometer;
use super::ProxyMsg;

pub struct ProxyEncoder{
  speed:Arc<Mutex<Tachometer>>
}

impl ProxyEncoder{
  pub fn new()->Self{
    Self { speed:Arc::new(Mutex::new(Tachometer::new())) }
  }

  
}

impl Encoder<ProxyMsg> for ProxyEncoder{
    fn encode(&mut self,ctx:&mut east_core::context2::Context<ProxyMsg>,msg:ProxyMsg,byte_buf:&mut east_core::byte_buf::ByteBuf) {
      let mut s=self.speed.lock().unwrap();
      if s.has(msg.buf.len()){
        let speed:f64=s.speed() as f64/1024f64;
        // log::info!("speed->{:.2}kb/s",speed);
      }
      byte_buf.write_bytes(&msg.buf);
    }
}