use std::{sync::Arc, collections::HashMap};

use schemars::schema::RootSchema;
use serde::{de::DeserializeOwned, Serialize, Deserialize};

use self::{server::Server, agent::Agent};


pub mod server;
pub mod agent;

const CONFIG_PATH:&str="conf.yml";


lazy_static!{
    pub static ref CONF:Arc<GlobalConfig>=Arc::new(load_config::<GlobalConfig>(CONFIG_PATH));
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig{
    #[serde(default = "default_server")]
    pub server:Server,
    pub agent:HashMap<String,Vec<Agent>>,
}

fn default_server()->Server{
    Server { bind: String::from("127.0.0.1:3555") }
}

fn load_config<T>(path: &str) -> T where T: DeserializeOwned {
    let root_schema=serde_yaml::from_str::<RootSchema>(&std::fs::read_to_string(path).expect(&format!("failure read file {}", path))).unwrap();

    let data = serde_json::to_string_pretty(&root_schema).expect("failure to parse RootSchema");
    let config = serde_json::from_str::<T>(&*data).expect(&format!("failure to format json str {}",&data));
    config

}

