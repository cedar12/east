#[derive(Debug,Clone)]
pub struct Proxy{
  pub bind_port:u16,
  pub target_host:String,
  pub target_port:u16,
  pub enable:bool,
  pub whitelist:Vec<String>
}