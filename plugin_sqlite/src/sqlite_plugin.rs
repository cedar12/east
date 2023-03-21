
use std::sync::{Arc, Mutex};

use east_plugin::{plugin::{DatabasePlugin, Plugin, Type}, agent::Agent, proxy::Proxy};

use crate::db::{self, CONN};

#[derive(Clone)]
pub struct SqlitePlugin;
impl DatabasePlugin for SqlitePlugin {
    fn get_agents(&self)->anyhow::Result<Vec<east_plugin::agent::Agent>> {
        let mut agents=vec![];
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("SELECT id, name FROM agent")?;
            let agent_iter = stmt.query_map([], |row| {
                Ok(Agent {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    proxy: vec![],
                })
            })?;
            
            for agent in agent_iter {
                agents.push(agent?);
            }
            for (_,agent) in agents.iter_mut().enumerate(){
                let id=agent.clone().id;
                let mut stmt=conn.prepare("select bind_port,target_host,target_port,enable,whitelist from proxy where agent_id=?").unwrap();
                let proxy_iter = stmt.query_map([id], |row| {
                    let whitelist:String=row.get(4)?;
                    let v:Vec<&str>=whitelist.split(",").collect();
                    let v:Vec<String>=v.iter().map(|f|f.to_string()).collect();
                    Ok(Proxy {
                        bind_port: row.get(0)?,
                        target_host: row.get(1)?,
                        target_port: row.get(2)?,
                        enable: row.get(3)?,
                        whitelist: v,
                    })
                });
                if let Ok(proxy_iter)=proxy_iter{
                    for proxy in proxy_iter{
                        let proxy=proxy?;
                        agent.proxy.push(proxy);
                    }
                }
            }
        }
        
        Ok(agents)
    }

    fn config(&self,conf:east_plugin::plugin::DBConfig) ->anyhow::Result<()> {
        db::init(conf)
    }


    fn set_proxy_enable(&self,bind_port:u16,enable:bool)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("update proxy enable=? where bind_port=?")?;
            stmt.execute((enable,bind_port))?;
        }
        Ok(())
    }

    fn add_agent(&self,agent:Agent)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("insert into agent values(?,?)")?;
            stmt.execute((agent.id,agent.name))?;
        }
        Ok(())
    }

    fn remove_agent(&self,id:String)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("delete from agent where id=?")?;
            stmt.execute([id])?;
        }
        Ok(())
    }

    fn add_proxy(&self,agent_id:String,proxy:Proxy)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let whitelist=proxy.whitelist.join(",");
            let mut stmt = conn.prepare("insert into proxy values(?,?,?,?,?,?)")?;
            stmt.execute((proxy.bind_port,agent_id,proxy.target_host,proxy.target_port,proxy.enable,whitelist))?;
        }
        Ok(())
    }

    fn remove_proxy(&self,bind_port:u16)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("delete from proxy where bind_port=?")?;
            stmt.execute([bind_port])?;
        }
        Ok(())
    }
}
impl Plugin for SqlitePlugin{

    fn version(&self)->String {
        "v0.0.1".into()
    }

    fn info(&self)->String {
        "sqlite database plugin".into()
    }

    fn author(&self)->String {
        "cedar12.zxd@qq.com".into()
    }

    fn plugin_type(&self)->east_plugin::plugin::Type {
        Type::DatabasePlugin
    }
    
}