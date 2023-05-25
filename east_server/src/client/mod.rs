use std::time::{Duration, self, SystemTime};

use east_core::{context::Context, message::Msg};



#[derive(Clone,Debug)]
struct Client{
  pub id:String,
  pub ctx:Context<Msg>,
  pub time:SystemTime,
}

impl Client {
    pub fn new(id:String,ctx:Context<Msg>)->Self{
      Self { id: id, ctx: ctx ,time:time::SystemTime::now() }
    }


}