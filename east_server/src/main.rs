#[macro_use]
extern crate lazy_static; 
extern crate east_core;
extern crate east_plugin;
extern crate libloading;

mod connection;
mod connection2;
mod server;
mod handler;

mod proxy;

mod config;

mod plugin;

mod tests;

use log::LevelFilter;
use log4rs::{append::{console::ConsoleAppender, file::FileAppender, rolling_file::{RollingFileAppender, policy::compound::{CompoundPolicy, trigger::size::SizeTrigger, roll::fixed_window::FixedWindowRoller}}}, encode::pattern::PatternEncoder, config::{Config,Appender, Root, Logger}};
use plugin::init_plugin;
use tokio::io::Result;


fn init_log(){
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[EAST] {d(%Y-%m-%d %H:%M:%S)} - {l} -{t} - {m}{n}")))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[EAST] {d(%Y-%m-%d %H:%M:%S)} - {l} - {t} - {m}{n}")))
        .build("log/east.log")
        .unwrap();
   

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
            .appender("file")
            .additive(true)
            
            .build("EAST", LevelFilter::Info))
        .build(Root::builder().appender("stdout").appender("file").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_log();
    init_plugin().await;
    server::run().await
}

