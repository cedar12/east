

use east_plugin::plugin::{DBConfig, WebPlugin, DatabasePlugin};

use crate::actix_plugin::ActixPlugin;



#[test]
fn test(){
    let ap=ActixPlugin{};
    let db=east_sqlite::sqlite_plugin::SqlitePlugin{};
    db.config(DBConfig{url:"../plugin_sqlite/data.db".into(),username:None,password:None});
    ap.run("127.0.0.1:8088".into(),Box::new(db));
}