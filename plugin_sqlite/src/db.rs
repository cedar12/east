use std::sync::{Mutex, Arc};

use anyhow::Ok;
use east_plugin::plugin::DBConfig;
use rusqlite::Connection;

const AGENT_TABLE_SQL:&str="
create table if not exists agent(
  id text primary key,
  name text not null
);
";
const PROXY_TABLE_SQL:&str="
create table if not exists proxy(
  bind_port integer primary key,
  agent_id text not null,
  target_host text not null,
  target_port integer not null,
  `enable` integer default '0',
  whitelist text default ''
);
";

lazy_static!{
  pub static ref CONN:Arc<Mutex<Option<Connection>>>=Arc::new(Mutex::new(None));
}


pub fn init(conf:DBConfig)->anyhow::Result<()>{
  let conn=Connection::open(conf.url)?;
  let mut c=CONN.lock().unwrap();
  let conn=c.insert(conn);
  conn.execute(AGENT_TABLE_SQL, ())?;
  conn.execute(PROXY_TABLE_SQL, ())?;
  Ok(())
}