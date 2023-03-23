use actix_files::{Files, FilesService};
use actix_web::{rt::Runtime, HttpServer, App,web};
use east_plugin::plugin::{Plugin, WebPlugin,DatabasePlugin, Type};

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
    fn run(&self,bind:String,dp:Box<dyn DatabasePlugin>)->anyhow::Result<()> {
        let rt = Runtime::new()?;
        let dp=web::Data::new(dp);
        let run=HttpServer::new(move|| {
            App::new().app_data(dp.clone())
            // .service(web::resource("/").route(web::get().to(route::index)))
            .service(route::agents)
            .service(route::add_agent)
            .service(route::remove_agent)
            .service(route::add_proxy)
            .service(Files::new("/","./static/").index_file("index.html"))
        })
        .bind(bind)?
        .run();
        rt.block_on(run)?;
        Ok(())
    }
}