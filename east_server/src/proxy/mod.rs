use std::sync::{Arc, atomic::{AtomicUsize, Ordering, AtomicU64}};

use anyhow::{Result, Ok};
use tokio::{net::{TcpListener, TcpStream}, io::{split, ReadHalf,WriteHalf, AsyncReadExt}, spawn};


pub struct Proxy{
  addr:String,
  listen:Arc<Option<TcpListener>>,
  last_id:AtomicU64,
}

impl Proxy{
  pub fn new(addr:String)->Self{
    Proxy{
      addr:addr,
      listen:Arc::new(None),
      last_id:AtomicU64::new(1),
    }
  }

  pub async fn listen(&mut self)->Result<()>{
    let listen=TcpListener::bind(self.addr.as_str()).await?;
    self.listen=Arc::new(Some(listen));
    Ok(())
  }

  pub async fn accpet<F>(&self,f:F)->Result<()> where F:FnOnce(u64,Vec<u8>){
    let l=Arc::clone(&self.listen);
    if let Some(listen)=l.as_ref(){
      loop{
        let (stream,addr)=listen.accept().await?;
        let (mut r,w)=split(stream);
        let id=self.last_id.load(Ordering::Relaxed);
        self.last_id.store(id+1, Ordering::Relaxed);
        spawn(async move{
          loop{
            let mut buf=vec![0u8;64];
            let n=r.read(&mut buf).await.unwrap();
            if n==0{
              return;
            }
            f(id,buf[..n].to_vec());
          }
        });
      }
    }
    Ok(())
  }
}
