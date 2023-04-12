use std::{sync::Arc, collections::HashMap, env};

use schemars::schema::RootSchema;
use serde::{de::DeserializeOwned, Serialize, Deserialize};

use self::{server::{Server, Plugin, default_plugin, default_bind, default_database, default_web}, agent::Agent};


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
    #[serde(default = "default_agent")]
    pub agent:HashMap<String,Vec<Agent>>,
}

fn default_server()->Server{
    Server { bind: default_bind(),plugin:default_plugin()}
}

fn default_agent()->HashMap<String,Vec<Agent>>{
    HashMap::new()
}

fn load_config<T>(path: &str) -> T where T: DeserializeOwned {
    let mut f_path=String::from(path);
    if let Some(p)=get_args_path(){
        f_path=p;
    }
    let f=std::fs::read_to_string(f_path).unwrap();
    let root_schema=serde_yaml::from_str::<RootSchema>(&f).unwrap();

    let data = serde_json::to_string_pretty(&root_schema).expect("failure to parse RootSchema");
    let config = serde_json::from_str::<T>(&*data).expect(&format!("failure to format json str {}",&data));
    config

}

fn get_args_path()->Option<String>{
    let args:Vec<String>=std::env::args().collect();
    if args.len()==2{
        let a = &args[1];
        return Some(String::from(a));
    }
    None
}

