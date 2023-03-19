use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    #[serde(default = "default_bind")]
    pub bind: String,
}

fn default_bind()->String{
    String::from("127.0.0.1:3555")
}


