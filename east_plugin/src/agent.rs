use serde::Serialize;

use crate::proxy::Proxy;

#[derive(Debug,Clone,Serialize)]
pub struct Agent{
  pub id:String,
  pub name:String,
  pub proxy:Vec<Proxy>
}