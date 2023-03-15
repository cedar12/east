use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

pub struct Context<T> {
    in_tx: Arc<Mutex<Sender<T>>>,
    out_tx: Arc<Mutex<Sender<T>>>,
    attributes:Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn Any + Send + Sync>>>>>>,
    addr:SocketAddr,
}

impl<T> Debug for Context<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").field("addr", &self.addr).finish()
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
            attributes:self.attributes.clone(),
            addr:self.addr.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<T> Context<T> {
    pub fn new(in_tx: Sender<T>, out_tx: Sender<T>,addr:SocketAddr) -> Self {
        Context {
            in_tx: Arc::new(Mutex::new(in_tx)),
            out_tx: Arc::new(Mutex::new(out_tx)),
            attributes:Arc::new(Mutex::new(HashMap::new())),
            addr
        }
    }

    

    #[allow(dead_code)] 
    pub fn addr(&self)->SocketAddr{
      self.addr
    }

    pub fn out(&self, msg: T)
    where
        T: Send + 'static,
    {
        let s=self.out_tx.lock().unwrap();
        s.send(msg).unwrap();
    }
    pub fn write(&self, msg:T){
        let s=self.in_tx.lock().unwrap();
        s.send(msg).unwrap();
    }



}