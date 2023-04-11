use std::{sync::Arc, any::{Any, TypeId}, path::{PathBuf, Path}};

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
        },
        Type::WebPlugin=>{}
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
        whitelist: vec!["192.168.1.0/24".into(),"127.0.0.1/16".into()],
    };
    let r=a.match_addr("127.0.1.1".into());
    assert_eq!(true,r)
}

#[tokio::test]
async fn test_plugin_manage(){
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path=dir.as_path().parent().unwrap();
    let mut pm=plugin::manage::PluginManager::new();
    pm.init_plugin_dir(Path::new("plugin")).await;
    println!("{:?}",pm.plugins());
    if let Some((name,pi))=pm.get_plugin_by_type(Type::DatabasePlugin){
        println!("{:?}->{:?}",name,pi);
        let plugin=pm.call_plugin_db(name).await;
        if let Some(plugin)=plugin{
            // plugin.config(config::CONF.server.plugin.database.clone().db_config()).unwrap();
            println!("{:?}",plugin.get_agents());
        }
        
    }
    
}


#[tokio::test]
async fn test_rate_limiter(){
    use east_core::token_bucket::TokenBucket;
    let bucket = TokenBucket::new(128.0*1024.0, 1024*1024);
    // let start = tokio::time::Instant::now();
    // for i in 0..10 {
    //     bucket.take(i+2020 + 1).await;
    //     println!("{}: {:?}", i + 1, start.elapsed());
    // }
    bucket.take(128*1024+1).await;

}
