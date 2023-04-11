use east_plugin::plugin::DBConfig;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Server {
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_plugin")]
    pub plugin:Plugin,
}

pub fn default_plugin()->Plugin{
    Plugin { dir: default_plugin_dir(), database: default_database(), web: default_web() }
}

pub fn default_bind()->String{
    String::from("127.0.0.1:3555")
}

pub fn default_plugin_dir()->String{
    "plugin".into()
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Plugin{
    #[serde(default="default_plugin_dir")]
    pub dir:String,
    #[serde(default="default_database")]
    pub database:Database,
    #[serde(default="default_web")]
    pub web:Web,
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
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Web{
    #[serde(default="default_web_bind")]
    pub bind:String,
    #[serde(default="default_web_username")]
    pub username:String,
    #[serde(default="default_web_password")]
    pub password:String
}

fn default_web_bind()->String{
    "127.0.0.1:8088".into()
}

fn default_web_username()->String{
    "east".into()
}

fn default_web_password()->String{
    "East&*!2023".into()
}
pub fn default_web()->Web{
    Web { bind: default_web_bind(),username:default_web_username(),password:default_web_password() }
}