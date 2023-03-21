use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default="default_plugin")]
    pub plugin:String,
}

fn default_bind()->String{
    String::from("127.0.0.1:3555")
}

fn default_plugin()->String{
    "plugin".into()
}

