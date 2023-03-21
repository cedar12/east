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

mod tests;

use log::LevelFilter;
use log4rs::{append::{console::ConsoleAppender, file::FileAppender, rolling_file::{RollingFileAppender, policy::compound::{CompoundPolicy, trigger::size::SizeTrigger, roll::fixed_window::FixedWindowRoller}}}, encode::pattern::PatternEncoder, config::{Config,Appender, Root, Logger}};
use tokio::io::Result;


fn init_log(){
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[EAST] {d} - {l} -{t} - {m}{n}")))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[EAST] {d} - {l} - {t} - {m}{n}")))
        .build("log/east.log")
        .unwrap();
    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder().build("log{}",window_size).unwrap();
    let size_limit = 5 * 1024; // 5KB as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);
    let policy=CompoundPolicy::new(Box::new(size_trigger),Box::new(fixed_window_roller));

   

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
            .appender("file")
            .additive(false)
            
            .build("EAST", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_log();
    server::run().await
}

