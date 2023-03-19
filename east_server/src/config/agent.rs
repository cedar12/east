use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Agent{
   pub bind_port:u16,
   #[serde(default = "default_host")]
   pub target_host:String,
   pub target_port:u16, 
}

fn default_host() -> String{
    String::from("127.0.0.1")
}