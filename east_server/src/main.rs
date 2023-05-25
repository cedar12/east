#[macro_use]
extern crate lazy_static; 
extern crate east_core;
extern crate east_plugin;
extern crate libloading;
extern crate cron_job;

mod connection;
mod server;
mod handler;

mod proxy;

mod config;

mod plugin;

mod control;
mod log_conf;
mod key;
mod client;


#[cfg(test)]
mod tests;

use plugin::init_plugin;
use tokio::io::Result;


#[tokio::main]
async fn main() -> Result<()> {
    log_conf::init();
    let version: &'static str = env!("CARGO_PKG_VERSION");
    log::info!("Version: {}", version);
    let author: &'static str = env!("CARGO_PKG_AUTHORS");
    log::info!("Author: {}", author);
    key::init();
    init_plugin().await;
    tokio::spawn(async{
        proxy::proxy_signal().await;
    });
    tokio::spawn(async{
        if let Err(e)=std::fs::create_dir_all("./tmp"){
            log::error!("Failed to create a temp directory. {:?}",e)
        }
        connection::file_signal().await;
    });
    server::run().await
}

