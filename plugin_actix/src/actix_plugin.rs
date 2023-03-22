use actix_web::{rt::Runtime, HttpServer, App};
use east_plugin::plugin::{Plugin, WebPlugin, Type};

use crate::route::greet;



pub struct ActixPlugin;

impl Plugin for ActixPlugin{
    fn version(&self)->String {
        "v0.0.1".into()
    }

    fn info(&self)->String {
        "web plugin".into()
    }

    fn author(&self)->String {
        "cedar12.zxd@qq.com".into()
    }

    fn plugin_type(&self)->east_plugin::plugin::Type {
        Type::WebPlugin
    }
}

impl WebPlugin for ActixPlugin {
    fn run(&self)->anyhow::Result<()> {
        let rt = Runtime::new().unwrap();
        let run=HttpServer::new(|| {
            App::new().service(greet)
        })
        .bind(("127.0.0.1", 8080)).unwrap()
        .run();
        rt.block_on(run).unwrap();
        Ok(())
    }
}