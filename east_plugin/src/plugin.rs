
use crate::{agent::Agent, proxy::Proxy, control::{AgentControl, ProxyControl}};

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
pub trait Plugin: Send + Sync {
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
  fn modify_agent(&self,agent:Agent)->anyhow::Result<()>;
  fn get_proxys(&self,agent_id:String)->anyhow::Result<Vec<Proxy>>;
  fn get_proxy(&self,bind_port:u16)->anyhow::Result<(String,Proxy)>;
  fn add_proxy(&self,agent_id:String,proxy:Proxy)->anyhow::Result<()>;
  fn remove_proxy(&self,bind_port:u16)->anyhow::Result<()>;
  fn modify_proxy(&self,proxy:Proxy)->anyhow::Result<()>;
  fn set_proxy_enable(&self,bind_port:u16,enable:bool)->anyhow::Result<()>;

  fn get_user(&self,username:String)->anyhow::Result<(String,String)>;
  fn add_user(&self,username:String,password:String)->anyhow::Result<()>;
  fn remove_user(&self,username:String)->anyhow::Result<()>;
  fn modify_user(&self,username:String,password:String)->anyhow::Result<()>;
  
}

pub trait WebPlugin:Plugin{
  fn run(&self,bind:String,dp:Box<dyn DatabasePlugin>,control:(Box<dyn AgentControl>,Box<dyn ProxyControl>),account:(String,String))->anyhow::Result<()>;
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum Type {
  DatabasePlugin,
  WebPlugin,
}

pub struct DBConfig{
  pub url:String,
  pub username:Option<String>,
  pub password:Option<String>
}

pub trait WebPlugin2:Plugin{
  fn run(&self,bind:String,control:std::collections::HashMap<String,Box<dyn std::any::Any>>)->anyhow::Result<()>;
}