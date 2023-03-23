use std::sync::Arc;

use crate::{agent::Agent, proxy::Proxy};

///
/// 插件信息
/// ```
/// struct YourPlugin{}
/// impl Plugin for YourPlugin{
/// 
/// fn version(&self)->String {
///   "v0.0.1".into()
/// }
/// 
/// fn info(&self)->String {
///   "your plugin info".into()
/// }
/// 
/// fn author(&self)->String {
///   "your name".into()
/// }
/// 
/// fn plugin_type(&self)->east_plugin::plugin::Type {
///   Type::DatabasePlugin
/// }
/// 
/// }
/// #[no_mangle]
/// pub extern "C" fn install() -> *mut dyn Plugin {
///   Box::into_raw(Box::new(YourPlugin)) as *mut dyn Plugin
/// }
/// ```
/// 
pub trait Plugin: Send + Sync{
  fn version(&self)->String;
  fn info(&self)->String;
  fn author(&self)->String;
  fn plugin_type(&self)->Type;
}


///
/// 数据库插件
/// ```
/// struct YourPlugin{}
/// 
/// impl DatabasePlugin for YourPlugin{
/// }
/// 
/// #[no_mangle]
/// pub extern "C" fn create() -> *mut dyn DatabasePlugin {
///   Box::into_raw(Box::new(YourPlugin)) as *mut dyn DatabasePlugin
/// }
/// ```
pub trait DatabasePlugin:Plugin{
  fn config(&self,conf:DBConfig)->anyhow::Result<()>;
  fn get_agents(&self)->anyhow::Result<Vec<Agent>>;
  fn get_agent(&self,id:String)->anyhow::Result<Agent>;
  fn add_agent(&self,agent:Agent)->anyhow::Result<()>;
  fn remove_agent(&self,id:String)->anyhow::Result<()>;
  fn get_proxy(&self,bind_port:u16)->anyhow::Result<Proxy>;
  fn add_proxy(&self,agent_id:String,proxy:Proxy)->anyhow::Result<()>;
  fn remove_proxy(&self,bind_port:u16)->anyhow::Result<()>;
  fn set_proxy_enable(&self,bind_port:u16,enable:bool)->anyhow::Result<()>;
  
}

pub trait WebPlugin:Plugin{
  fn run(&self,bind:String,dp:Box<dyn DatabasePlugin>)->anyhow::Result<()>;

}

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum Type {
  DatabasePlugin,
  WebPlugin,
}

pub struct DBConfig{
  pub url:String,
  pub username:Option<String>,
  pub password:Option<String,>
}