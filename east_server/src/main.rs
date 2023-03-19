#[macro_use]
extern crate lazy_static; 
extern crate east_core;

mod connection;
mod connection2;
mod server;
mod handler;

mod proxy;

mod config;

mod tests;

use tokio::io::Result;


#[tokio::main]
async fn main() -> Result<()> {
    server::run().await
}

