use std::net::SocketAddr;
// use tokio::net::tcp::{ReadHalf, WriteHalf};
// use tokio::{sync::{mpsc::{Sender, SendError}, Mutex, Arc}, net::{TcpStream, SocketAddr, Shutdown}};
use tokio::sync::{Mutex};
use tokio::sync::mpsc::{Sender};
use tokio::net::TcpStream;
// use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::io::{self,AsyncReadExt,AsyncWriteExt,ReadHalf,WriteHalf,BufWriter};


pub struct Context<T> {
    in_tx: Mutex<Sender<T>>,
    out_tx: Mutex<Sender<T>>,
    addr:SocketAddr,
    // stream:Arc<Mutex<TcpStream>>,
    // w:Arc<Mutex<WriteHalf<TcpStream>>>,
    // r:Arc<Mutex<ReadHalf<TcpStream>>>,
}

impl<T> Context<T> {
    pub fn new(in_tx: Sender<T>, out_tx: Sender<T>,addr:SocketAddr) -> Self {
      // let (r,w)=io::split(stream);
      
        Context {
            in_tx: Mutex::new(in_tx),
            out_tx: Mutex::new(out_tx),
            addr
            // stream:Arc::new(Mutex::new(stream)),
            // w:Arc::new(Mutex::new(w)),
            // r:Arc::new(Mutex::new(r)),
        }
    }

    

    #[allow(dead_code)] 
    pub fn addr(&self)->SocketAddr{
      self.addr
    }

    // #[allow(dead_code)] 
    // pub async fn local_addr(&self)->io::Result<SocketAddr>{
    //   let s=self.stream.lock().await;
    //   s.local_addr()
    // }

    // #[allow(dead_code)] 
    // pub async fn shutdown(&self)->io::Result<()>{
    //   let mut s=self.stream.lock().await;
    //   s.shutdown().await?;
    //   Ok(())
    // }

    pub async fn out(&self, msg: T)
    where
        T: Send + 'static,
    {
        let s=self.out_tx.lock().await;
        s.send(msg).await;
    }
    pub async fn write(&self, msg:T) {
        self.in_tx.lock().await.send(msg).await;
    }
}

unsafe impl<T> Send for Context<T>
where
    T: Send +Sync + 'static,
{}
