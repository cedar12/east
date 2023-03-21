use std::{fs, fmt::Debug, path::Path, clone, sync::Arc};

use east_plugin::plugin::{Plugin, DatabasePlugin, Type};
#[cfg(not(target_os="windows"))]
use libloading::os::unix::{Library, Symbol};
#[cfg(target_os="windows")]
use libloading::os::windows::{Library, Symbol};
use tokio::sync::Mutex;

use crate::config;


#[derive(Clone)]
pub struct  PluginInfo{
    pub filename:String,
    pub info:String,
    pub version:String,
    pub author:String,
    pub plugin_type:Type
}

impl Debug for PluginInfo{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginInfo").field("filename", &self.filename).field("info", &self.info).field("version", &self.version).field("author", &self.author).field("plugin_type", &self.plugin_type).finish()
    }
}

impl PluginInfo{
    fn new(plugin:Box<dyn Plugin>,filename:String)->Self{
        let info=plugin.info();
        let version=plugin.version();
        let author=plugin.author();
        let plugin_type=plugin.plugin_type();
        PluginInfo { filename: filename, info: info, version: version, author: author,plugin_type:plugin_type }
    }
}

fn load_lib(filename:&str)->anyhow::Result<Box<dyn Plugin>>{
    let dir=config::CONF.server.plugin.clone();
    let plugin_file=Path::new(dir.as_str()).join(filename);
    let lib = unsafe{Library::new(plugin_file)}?;
    let plugin_install: Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> = unsafe {
        lib.get(b"install")
    }?;
    let plugin = unsafe { Box::from_raw(plugin_install()) };
    Ok(plugin)
}
fn create_database(filename:&str)->anyhow::Result<Box<dyn DatabasePlugin>>{
    let dir=config::CONF.server.plugin.clone();
    let plugin_file=Path::new(dir.as_str()).join(filename);
    let lib = unsafe{Library::new(plugin_file)}?;
    let plugin_install: Symbol<unsafe extern "C" fn() -> *mut dyn DatabasePlugin> = unsafe {
        lib.get(b"create")
    }?;
    let plugin = unsafe { Box::from_raw(plugin_install()) };
    Ok(plugin)
}

pub fn list()->anyhow::Result<Vec<PluginInfo>>{
    let dir=config::CONF.server.plugin.clone();
    let paths = fs::read_dir(dir)?;
    let mut v=vec![];
    for path in paths {
        let file_name=path?.file_name();
        let file_name=file_name.to_str();
        if let Some(filename)=file_name{
            println!("load {:?}",filename);
            let plugin=load_lib(filename);
            match plugin{
                Ok(plugin)=>{
                    v.push(PluginInfo::new(plugin,filename.to_string()));
                },
                Err(e)=>{log::error!("{:?}",e)}
            }
        }
    }
    Ok(v)
}

pub fn init(path:String){

}

pub fn get_database_plugin(plugin:PluginInfo)->anyhow::Result<Box<dyn DatabasePlugin>>{
    let plugin=create_database(plugin.filename.clone().as_str())?;
    Ok(plugin)
}