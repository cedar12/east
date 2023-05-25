use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex};
use tokio::sync::mpsc::{Sender};

pub struct Context<T> {
    in_tx: Arc<Mutex<Sender<T>>>,
    out_tx: Arc<Mutex<Sender<T>>>,
    close_tx:Arc<Mutex<Sender<()>>>,
    attributes:Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn Any + Send + Sync>>>>>>,
    addr:SocketAddr,
    is_run:Arc<AtomicBool>,
}

impl<T> Debug for Context<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Context").field(&self.addr).finish()
    }
}

impl<T> PartialEq for Context<T>{
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}


impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Context {
            in_tx:self.in_tx.clone(),
            out_tx:self.out_tx.clone(),
            close_tx:self.close_tx.clone(),
            attributes:self.attributes.clone(),
            addr:self.addr.clone(),
            is_run:self.is_run.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<T> Context<T> {
    pub fn new(in_tx: Sender<T>, out_tx: Sender<T>,addr:SocketAddr,close_tx:Sender<()>) -> Self {
        Context {
            in_tx: Arc::new(Mutex::new(in_tx)),
            out_tx: Arc::new(Mutex::new(out_tx)),
            close_tx:Arc::new(Mutex::new(close_tx)),
            attributes:Arc::new(Mutex::new(HashMap::new())),
            addr,
            is_run:Arc::new(AtomicBool::new(true))
        }
    }

    

    #[allow(dead_code)] 
    pub fn addr(&self)->SocketAddr{
      self.addr
    }

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

    pub async fn close(&self) {
        self.is_run.store(false, Ordering::Relaxed);
        self.close_tx.lock().await.send(()).await.unwrap();
    }
    pub async fn close_run(&self) {
        self.is_run.store(false, Ordering::Relaxed);
        self.close_tx.lock().await.send(()).await.unwrap();
    }

    pub fn is_run(&self)->bool{
        self.is_run.load(Ordering::Relaxed)
    }

    pub async fn set_attribute(&self, key: String, value: Box<dyn Any + Send + Sync>) {
        let mut attributes = self.attributes.lock().await;
        attributes.insert(key, Arc::new(Mutex::new(value)));
    }

    pub async fn remove_attribute(&self, key: String) {
        let mut attributes = self.attributes.lock().await;
        attributes.remove(&key);
    }

    pub async fn get_attribute(&self, key: String) -> Arc<Mutex<Box<dyn Any + Send + Sync>>> {
        let attributes = self.attributes.lock().await;
        let v = attributes.get(key.as_str());
        match v{
            Some(v)=>v.clone(),
            None=>Arc::new(Mutex::new(Box::new(())))
        }
    }

}

