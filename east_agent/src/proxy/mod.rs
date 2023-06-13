use std::{sync::{Arc}, collections::HashMap};
use tokio::sync::{Mutex, RwLock};

use east_core::context::Context;

pub mod proxy_decoder;
pub mod proxy_encoder;
pub mod proxy_handler;

lazy_static!{
  pub static ref ProxyMap:Arc<RwLock<HashMap<u64,Context<Vec<u8>>>>>=Arc::new(RwLock::new(HashMap::new()));
}

pub struct Proxy{

}