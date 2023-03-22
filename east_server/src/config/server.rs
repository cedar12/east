use east_plugin::plugin::DBConfig;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Server {
    #[serde(default = "default_bind")]
    pub bind: String,
    
    pub plugin:Plugin,
}

pub fn default_bind()->String{
    String::from("127.0.0.1:3555")
}

pub fn default_plugin()->String{
    "plugin".into()
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Plugin{
    #[serde(default="default_plugin")]
    pub dir:String,
    pub database:Database,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Database{
    pub url:String,
    pub username:Option<String>,
    pub password:Option<String>,
}

pub fn default_database()->Database{
    Database { url: "".into(), username: None, password: None }
}

impl Database{
    pub fn db_config(self)->DBConfig{
        DBConfig { url: self.url, username: self.username, password: self.password }
    }
}