use crate::{agent::Agent, proxy::Proxy};

pub trait Plugin{
  fn version(&self)->String;
  fn info(&self)->String;
  fn author(&self)->String;
  fn plugin_type(&self)->Type;
}


pub trait DatabasePlugin:Plugin{
  fn config(&self,conf:DBConfig)->anyhow::Result<()>;
  fn get_agents(&self)->anyhow::Result<Vec<Agent>>;
  fn add_agent(&self,agent:Agent)->anyhow::Result<()>;
  fn remove_agent(&self,id:String)->anyhow::Result<()>;
  fn add_proxy(&self,agent_id:String,proxy:Proxy)->anyhow::Result<()>;
  fn remove_proxy(&self,bind_port:u16)->anyhow::Result<()>;
  fn set_proxy_enable(&self,bind_port:u16,enable:bool)->anyhow::Result<()>;
  
}

pub enum Type {
  DatabasePlugin,
}

pub struct DBConfig{
  pub url:String,
  pub username:Option<String>,
  pub password:Option<String,>
}