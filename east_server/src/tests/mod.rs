use std::{sync::Arc, any::{Any, TypeId}, path::{PathBuf, Path}, time::Duration, thread, str::FromStr};

use east_plugin::plugin::{Plugin, DatabasePlugin, Type, DBConfig};
use tokio::{fs::File, io::{self, AsyncReadExt}};


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
        max_rate:None,
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

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs8::{EncodePrivateKey, LineEnding, EncodePublicKey, DecodePublicKey, DecodePrivateKey}};
#[test]
fn test_rsa(){
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let private_key_pem=priv_key.to_pkcs8_pem(LineEnding::CRLF).unwrap();
    let mut file = std::fs::File::create("private_key1.pem").expect("Failed to create private key file");
    std::io::Write::write_all(&mut file, private_key_pem.as_bytes()).expect("Failed to write private key to file");
    let public_key_pem=pub_key.to_public_key_pem(LineEnding::CRLF).unwrap();
    let mut file = std::fs::File::create("public_key1.pem").expect("Failed to create public key file");
    std::io::Write::write_all(&mut file, public_key_pem.as_bytes()).expect("Failed to write public key to file");

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}

#[test]
fn test_read_pem(){
    let mut rng = rand::thread_rng();
    let data = b"hello world";
    let pub_key=RsaPublicKey::read_public_key_pem_file("public_key1.pem").unwrap();
    let priv_key=RsaPrivateKey::read_pkcs8_pem_file("private_key.pem").unwrap();

    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");

    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    let d=String::from_utf8(dec_data.clone()).unwrap();
    println!("{}",d);
    assert_eq!(&data[..], &dec_data[..]);

}