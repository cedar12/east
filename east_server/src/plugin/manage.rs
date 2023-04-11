use east_plugin::plugin::{DatabasePlugin, Plugin, Type, WebPlugin};
use libloading::{Library, Symbol};
use std::fs;
use std::future::Future;
use std::path::Path;
use std::rc::Rc;
use std::{collections::HashMap, sync::Arc};

use crate::config;
use crate::control::{AgentControlImpl, ProxyControlImpl};

#[cfg(target_os = "windows")]
const PLUGIN_SUFFIX: &str = "dll";
#[cfg(target_os = "linux")]
const PLUGIN_SUFFIX: &str = "so";
#[cfg(target_os = "macos")]
const PLUGIN_SUFFIX: &str = "dylib";

#[derive(Clone)]
pub struct PluginInfo {
    pub lib: Arc<Library>,
    pub info: String,
    pub version: String,
    pub author: String,
    pub plugin_type: Type,
}

impl std::fmt::Debug for PluginInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("\n{:=<40}\ninfo: {}\ntype: {:?}\nversion: {}\nauthor: {}\n{:=<40}","=",self.info,self.plugin_type,self.version,self.author,"=").as_str())
    }
}

impl PluginInfo {
    fn new(plugin: Box<dyn Plugin>, lib: Library) -> Self {
        let info = plugin.info();
        let version = plugin.version();
        let author = plugin.author();
        let plugin_type = plugin.plugin_type();
        PluginInfo {
            lib: Arc::new(lib),
            info: info,
            version: version,
            author: author,
            plugin_type: plugin_type,
        }
    }
}

// 插件管理器
pub struct PluginManager {
    plugins: HashMap<String, PluginInfo>,
}

impl PluginManager {
    // 创建插件管理器
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    // 添加插件
    pub async fn add_plugin(&mut self, path: &Path) -> bool {
        match unsafe { Library::new(path) } {
            Ok(lib) => {
                let plugin_create: Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> =
                    unsafe { lib.get(b"install") }.unwrap();
                let plugin = unsafe { Box::from_raw(plugin_create()) };
                let pi = PluginInfo::new(plugin, lib);
                let name=String::from(path.file_stem().unwrap().to_str().unwrap());
                log::info!("加载插件->{}{:?}",name,pi);
                let namec=name.clone();
                let pic=pi.clone();
                self.plugins.insert(
                    name,
                    pi,
                );
                let ret=self.database_config(namec.as_str(),pic.clone()).await;
                if let Err(e)=ret{
                    log::error!("{:?}",e);
                }
                true
            }
            Err(_) => false,
        }
    }

    pub async fn init_web_run(&self){
      // self.web_run(namec.as_str(), pic.clone()).await;
      let a=self.get_plugin_by_type(Type::DatabasePlugin);
      if let Some((name,pi))=a{
        let db=self.call_plugin_db(name).await;
        if let Some(db_plugin)=db{
          let web=self.get_plugin_by_type(Type::WebPlugin);
          if let Some((name,pi))=web{
            let web=self.call_plugin_web(name).await;
            if let Some(web_plugin)=web{
              let bind=config::CONF.server.plugin.web.bind.clone();
              log::info!("Web插件[{}]开始启动监听->{}",name,bind);
              tokio::spawn(async move {
                let account=(config::CONF.server.plugin.web.username.clone(),config::CONF.server.plugin.web.password.clone());
                web_plugin.run(bind, db_plugin,(Box::new(AgentControlImpl::new()),Box::new(ProxyControlImpl::new())),account).unwrap();
              });
              
            }
          }
        }
        
      }
      

    }


    async fn database_config(&self,name:&str,pi:PluginInfo)->anyhow::Result<()>{
      if pi.plugin_type==Type::DatabasePlugin{
        let p=self.call_plugin_db(name).await;
        if let Some(p)=p{
          let db_config=config::CONF.server.plugin.database.clone().db_config();
          return p.config(db_config)
        }
      }
      Ok(())
    }

    // async fn web_run(&self,name:&str,pi:PluginInfo)->anyhow::Result<()>{
    //     if pi.plugin_type==Type::WebPlugin{
    //         let p=self.call_plugin_web(name).await;
    //         if let Some(p)=p{
    //           let h=tokio::spawn(async move {
    //             p.run().unwrap();
    //           });
    //         }
    //       }
    //     Ok(())
    // }

    pub fn get_plugin(&self, name: &str) -> Option<&PluginInfo> {
        self.plugins.get(name)
    }

    pub fn get_plugin_by_type(&self, plugin_type: Type) -> Option<(&String, &PluginInfo)> {
        for (name, pi) in self.plugins.iter() {
            if pi.plugin_type == plugin_type {
                return Some((name, pi));
            }
        }
        None
    }

    // 移除插件
    pub fn remove_plugin(&mut self, name: &str) -> bool {
        match self.plugins.remove(name) {
            Some(lib) => true,
            None => false,
        }
    }

    // 获取插件列表
    pub fn plugins(&self) -> Vec<String> {
        self.plugins.keys().map(|key| key.clone()).collect()
    }

    // 调用插件
    pub async fn call_plugin_db(&self, name: &str) -> Option<Box<dyn DatabasePlugin>> {
        if let Some(pi) = self.plugins.get(name) {
            let lib = Arc::clone(&pi.lib);
            if pi.plugin_type == Type::DatabasePlugin {
                let plugin_create: Symbol<unsafe extern "C" fn() -> *mut dyn DatabasePlugin> =
                    unsafe { lib.get(b"create") }.unwrap();
                let plugin = unsafe { Box::from_raw(plugin_create()) };
                return Some(plugin);
            }
            
        }
        None
    }

    pub async fn call_plugin_web(&self, name: &str)-> Option<Box<dyn WebPlugin>> {
        if let Some(pi) = self.plugins.get(name) {
            let lib = Arc::clone(&pi.lib);
            if pi.plugin_type == Type::WebPlugin{
                let plugin_create: Symbol<unsafe extern "C" fn() -> *mut dyn WebPlugin> =
                    unsafe { lib.get(b"create") }.unwrap();
                let plugin = unsafe { Box::from_raw(plugin_create()) };
                return Some(plugin);
            }
        }
        None
    }

    // 初始化插件目录
    pub async fn init_plugin_dir(&mut self, dir: &Path) -> bool {
        if !dir.exists() || !dir.is_dir() {
            return false;
        }
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == PLUGIN_SUFFIX {
                self.add_plugin(&path).await;
            }
        }
        true
    }
}
