use std::sync::Arc;

use east_core::context2::Context;
use east_core::message::Msg;
use std::sync::Mutex;

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
}

#[derive(Debug)]
pub struct Connections{
    conns:Arc<Mutex<Vec<Connection>>>
}


impl Connections {
    pub fn new()->Self{
        Connections { conns: Arc::new(Mutex::new(Vec::new())) }
    }
    pub fn push(&self,client:Connection){
        let mut conns=self.conns.lock().unwrap();
        conns.push(client);
    }
    pub fn remove(&self,ctx:Context<Msg>)->bool{
        let mut conns=self.conns.lock().unwrap();
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
    pub fn get(&self,id:String)->Option<Connection>{
        let conns=self.conns.lock().unwrap();
        let item=conns.iter().find(|&x|x.id==id);
        match item{
            Some(c)=>Some(c.clone()),
            None=>None
        }
    }

    pub fn println(&self){
        let conns=self.conns.lock().unwrap();
        println!("{:?}",conns);
    }
}

