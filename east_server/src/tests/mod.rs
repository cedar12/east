use std::{sync::Arc, any::{Any, TypeId}, path::PathBuf};

use east_plugin::plugin::{Plugin, DatabasePlugin, Type, DBConfig};


use crate::{config::{self, agent::Agent}, plugin};

#[cfg(target_os="windows")]
use libloading::os::windows::{Symbol, Library};
#[cfg(not(target_os="windows"))]
use libloading::os::unix::{Symbol,Library};


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
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("plugin");
    #[cfg(target_os="macos")]
    dir.push("east_sqlite.dylib");
    #[cfg(target_os="windows")]
    dir.push("east_sqlite.dll");
    // 加载动态链接库
   
    // let lib = unsafe{Library::new("./plugin/east_sqlite.dll")}.unwrap();
    
    let lib = unsafe{Library::new(dir)}.unwrap();
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
            let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let path=dir.as_path().parent().unwrap();
            plugin.config(DBConfig{
                url: path.join("plugin_sqlite").join("data.db").to_str().expect("").into(),
                username: None,
                password: None,
            }).unwrap();
            println!("{:?}",plugin.get_agents());
        }
    }
    
}

#[test]
fn test_dir(){
    let plugins=plugin::list().unwrap();
    println!("{:?}",plugins);
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let path=dir.as_path().parent().unwrap();
    let mut plugin=None;
    for p in plugins.iter(){
        match p.plugin_type{
            Type::DatabasePlugin=>{
                let p=plugin::get_database_plugin(p.clone()).unwrap();
                p.config(DBConfig { url: path.join("plugin_sqlite").join("data.db").to_str().expect("").into(), username: None, password: None }).unwrap();
                plugin.insert(p);
                break;
            }
        }
    }
    if let Some(plugin)=plugin{
        let agents=plugin.get_agents();
        println!("{:?}",agents);
    }
}

#[test]
fn test_ip(){
    let white:String="192.168.1.0/24".into();
    let v:Vec<&str>=white.split("/").collect();
    println!("{:?}",v);

    let a=Agent{
        bind_port: 2000,
        target_host: "".into(),
        target_port: 123,
        whitelist: vec!["192.168.1.0/24".into(),"127.0.0.1/32".into()],
    };
    let r=a.match_addr("127.0.1.1".into());
    assert_eq!(true,r)
}