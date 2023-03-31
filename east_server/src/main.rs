#[macro_use]
extern crate lazy_static; 
extern crate east_core;
extern crate east_plugin;
extern crate libloading;
extern crate cron_job;

mod connection;
mod connection2;
mod server;
mod handler;

mod proxy;

mod config;

mod plugin;

mod control;
mod log_conf;
mod tests;

use plugin::init_plugin;
use tokio::io::Result;


#[tokio::main]
async fn main() -> Result<()> {
    log_conf::init();
    init_plugin().await;
    tokio::spawn(async{
        proxy::proxy_signal().await;
    });
    server::run().await
}

