use std::{sync::Arc, any::{Any, TypeId}};

use east_plugin::plugin::{Plugin, DatabasePlugin, Type, DBConfig};
use libloading::{ os::windows::{Symbol, Library}};

use crate::config;



#[test]
fn test_conf(){
    let conf=Arc::clone(&config::CONF);
    println!("{:?}",conf);

}


#[test]
fn test_plugin(){
    let call_dynamic=|| -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let lib = libloading::Library::new("./plugin/east_sqlite.dll")?;
            let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"hello_rust")?;
            Ok(func())
        }
    };
    call_dynamic().unwrap();
}


#[test]
fn test_sqlite_plugin(){
    // 加载动态链接库
    let lib = unsafe{Library::new("./plugin/east_sqlite.dll")}.unwrap();
    // 获取插件实现
    let plugin_install: Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> = unsafe {
        lib.get(b"install")
    }.unwrap();
    let plugin = unsafe { Box::from_raw(plugin_install()) };
    // 使用插件
    println!("plugin info->{}",plugin.info());
    println!("plugin version->{}",plugin.version());
    println!("plugin author->{}",plugin.author());
    match plugin.plugin_type(){
        Type::DatabasePlugin=>{
            let plugin_create: Symbol<unsafe extern "C" fn() -> *mut dyn DatabasePlugin> = unsafe {
                lib.get(b"create")
            }.unwrap();
            let plugin = unsafe { Box::from_raw(plugin_create()) };
            plugin.config(DBConfig{
                url: "../plugin_sqlite/data.db".into(),
                username: None,
                password: None,
            }).unwrap();
            println!("{:?}",plugin.get_agents());
        }
    }
    
}