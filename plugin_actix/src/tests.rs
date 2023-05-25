

use std::any::{Any, TypeId};

use east_plugin::plugin::{DBConfig, WebPlugin, DatabasePlugin, Plugin, Type};

use crate::actix_plugin::ActixPlugin;



// #[test]
// fn test(){
//     let ap=ActixPlugin{};
//     let db=east_sqlite::sqlite_plugin::SqlitePlugin{};
//     db.config(DBConfig{url:"../plugin_sqlite/data.db".into(),username:None,password:None});
//     ap.run("127.0.0.1:8088".into(),Box::new(db));
// }


struct TestA{

}
impl Plugin for TestA {
    fn version(&self)->String {
        "".into()
    }

    fn info(&self)->String {
        "".into()
    }

    fn author(&self)->String {
        "".into()
    }

    fn plugin_type(&self)->east_plugin::plugin::Type {
        Type::WebPlugin
    }
}

impl WebPlugin for TestA{
    fn run(&self,bind:String,dp:Box<dyn DatabasePlugin>,control:(Box<dyn east_plugin::control::AgentControl>,Box<dyn east_plugin::control::ProxyControl>),account:(String,String))->anyhow::Result<()> {
        Ok(())
    }
}


#[test]
fn test_dyn(){
    let a: Box<dyn Plugin>=Box::new(TestA{});
    let b: Box<dyn WebPlugin> = unsafe { std::mem::transmute(a) };
    println!("{:?}",b.type_id())
    
}