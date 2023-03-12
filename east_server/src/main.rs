#[macro_use]
extern crate lazy_static; 
extern crate east_core;

mod connection;
mod server;

use connection::Connections;

use tokio::io::Result;

lazy_static! {
    // static ref SOCKETS: Mutex<Vec<Connection>> = Mutex::new(Vec::new());
    static ref CONNS:Connections=Connections::new();
}

#[tokio::main]
async fn main() -> Result<()> {
    server::run().await
}

