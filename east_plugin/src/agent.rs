use serde::{Serialize, Deserialize};

use crate::proxy::Proxy;

#[derive(Debug,Clone,Serialize,Deserialize)]
#[allow(non_snake_case)]
pub struct Agent{
  pub id:String,
  pub name:String,
  #[serde(default = "default_proxy")]
  pub proxy:Vec<Proxy>
}

fn default_proxy()->Vec<Proxy>{
  vec![]
}