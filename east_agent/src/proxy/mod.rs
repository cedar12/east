use std::{sync::{Arc}, collections::HashMap};
use tokio::sync::Mutex;

use east_core::context::Context;

pub mod proxy_decoder;
pub mod proxy_encoder;
pub mod proxy_handler;

lazy_static!{
  pub static ref ProxyMap:Arc<Mutex<HashMap<u64,Context<Vec<u8>>>>>=Arc::new(Mutex::new(HashMap::new()));
}

pub struct Proxy{

}