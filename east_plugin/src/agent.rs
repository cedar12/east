use crate::proxy::Proxy;

#[derive(Debug,Clone)]
pub struct Agent{
  pub id:String,
  pub name:String,
  pub proxy:Vec<Proxy>
}