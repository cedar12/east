use std::sync::Arc;

use east_core::context::Context;
use east_core::message::Msg;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref Conns:Connections=Connections::new();
}

#[derive(Clone,Debug)]
pub struct Connection{
    ctx:Context<Msg>,
    id:String,
}

impl Connection {
    pub fn new(ctx:Context<Msg>,id:String)->Self{
        Connection { ctx:ctx, id:id }
    }
    pub fn ctx(self)->Context<Msg>{
        self.ctx
    }
    pub fn id(self)->String{
        self.id
    }
}

#[derive(Debug)]
pub struct Connections{
    conns:Arc<Mutex<Vec<Connection>>>
}


impl Connections {
    pub fn new()->Self{
        Connections { conns: Arc::new(Mutex::new(Vec::new())) }
    }
    pub async fn push(&self,client:Connection){
        let mut conns=self.conns.lock().await;
        conns.push(client);
    }
    pub async fn remove(&self,ctx:Context<Msg>)->bool{
        let mut conns=self.conns.lock().await;
        let mut index=None;
        for (i,c) in conns.iter().enumerate() {
            if ctx==c.ctx{
                index=Some(i);
            }
        }
        match index{
            Some(i)=>{
                conns.remove(i);
                true
            },
            None=>{
                false
            }
        }
    }
    pub async fn get(&self,id:String)->Option<Connection>{
        let conns=self.conns.lock().await;
        let item=conns.iter().find(|&x|x.id==id);
        match item{
            Some(c)=>Some(c.clone()),
            None=>None
        }
    }

    pub async fn println(&self){
        let conns=self.conns.lock().await;
        println!("{:?}",conns);
    }
}

