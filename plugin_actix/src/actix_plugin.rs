use std::{sync::Mutex, collections::HashMap};

use actix_files::{Files};
use actix_web::{rt::Runtime, HttpServer, App,web};
use east_plugin::{plugin::{Plugin, WebPlugin,DatabasePlugin, Type}, control::AgentControl};


use crate::route;


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
    fn run(&self,bind:String,dp:Box<dyn DatabasePlugin>,ac:Box<dyn AgentControl>)->anyhow::Result<()> {
        let rt = Runtime::new()?;
        let dp=web::Data::new(dp);
        let ac=web::Data::new(ac);
        let locked_store=web::Data::new(Mutex::new(HashMap::<String,u64>::new()));
        
        let run=HttpServer::new(move|| {
            App::new().app_data(dp.clone())
            .app_data(ac.clone())
            .app_data(locked_store.clone())
            // .service(web::resource("/").route(web::get().to(route::index)))
            .service(web::scope("/api")
                .service(route::agents)
                .service(route::add_agent)
                .service(route::remove_agent)
                .service(route::proxys)
                .service(route::add_proxy)
                .service(route::remove_proxy)
                .service(route::modify_proxy)
                .service(route::login)
                .service(route::login_user_info)
            )
            .service(route::dist)
            .service(route::index)
            // .service(Files::new("/",asset_dir.metadata.into()).index_file("index.html"))
        })
        .bind(bind)?
        .run();
        rt.block_on(run)?;
        Ok(())
    }
}