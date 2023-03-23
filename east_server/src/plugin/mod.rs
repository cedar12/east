use std::{sync::Arc, path::Path};

use east_plugin::plugin::{DatabasePlugin, Type};
use tokio::sync::Mutex;

use crate::{config, plugin::manage::PluginManager};

use self::manage::PluginInfo;

pub mod manage;


lazy_static!{
    pub static ref PM:Arc<Mutex<PluginManager>>=Arc::new(Mutex::new(PluginManager::new()));
}

pub async fn init_plugin(){
    let dir=config::CONF.server.plugin.dir.clone();
    let mut pm=PM.lock().await;
    pm.init_plugin_dir(Path::new(dir.as_str())).await;
    pm.init_web_run().await;
}

pub async fn database_plugin()->anyhow::Result<(Box<dyn DatabasePlugin>,PluginInfo)>{
    let pm=PM.lock().await;
    let plugin=pm.get_plugin_by_type(Type::DatabasePlugin);
    if let Some((name,pi))=plugin{
        let p=pm.call_plugin_db(name).await;
        if let Some(p)=p{
            return Ok((p,pi.clone()))
        }
    }
    Err(anyhow::anyhow!(""))
}