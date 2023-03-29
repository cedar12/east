use serde::{Serialize, Deserialize};

use east_plugin::proxy::Proxy;

#[derive(Debug,Clone,Serialize,Deserialize)]
#[allow(non_snake_case)]
pub struct AgentModel{
  pub id:String,
  pub name:String,
  pub is_online:bool,
  #[serde(default = "default_proxy")]
  pub proxy:Vec<Proxy>
}

fn default_proxy()->Vec<Proxy>{
  vec![]
}