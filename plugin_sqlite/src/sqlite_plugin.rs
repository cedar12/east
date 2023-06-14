
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use east_plugin::{plugin::{DatabasePlugin, Plugin, Type}, agent::Agent, proxy::Proxy};
use rusqlite::Connection;

use crate::db::{self, CONN};

#[derive(Clone)]
pub struct SqlitePlugin;

impl DatabasePlugin for SqlitePlugin {
    fn get_agents(&self)->anyhow::Result<Vec<east_plugin::agent::Agent>> {
        let mut agents=vec![];
        // let conn=CONN.lock().unwrap();
        let conn=CONN.try_lock();
        match conn {
            Ok(conn) => {
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
                        let mut stmt=conn.prepare("select bind_port,target_host,target_port,enable,whitelist,max_rate from proxy where agent_id=?").unwrap();
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
                                max_rate: row.get(5)?,
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
            },
            Err(e) => {
                Err(anyhow::anyhow!("{:?}",e))
            },
        }
        
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
            let mut stmt = conn.prepare("insert into proxy values(?,?,?,?,?,?,?)")?;
            stmt.execute((proxy.bind_port,agent_id,proxy.target_host,proxy.target_port,proxy.enable,proxy.max_rate,whitelist))?;
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

    fn get_agent(&self,id:String)->anyhow::Result<Agent> {
        // let conn=CONN.lock().unwrap();
        let conn=CONN.try_lock();
        match conn {
            Ok(conn) => {
                if let Some(conn)=conn.as_ref(){
                    let mut stmt = conn.prepare("SELECT id, name FROM agent where id=?")?;
                    let agent_iter = stmt.query_map([id], |row| {
                        Ok(Agent {
                            id: row.get(0).unwrap(),
                            name: row.get(1).unwrap(),
                            proxy: vec![],
                        })
                    })?;
                    for a in agent_iter{
                        let mut agent=a?;
                        let id=agent.clone().id;
                        let mut stmt=conn.prepare("select bind_port,target_host,target_port,enable,whitelist,max_rate from proxy where agent_id=?").unwrap();
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
                                max_rate: row.get(5).unwrap(),
                            })
                        });
                        if let Ok(proxy_iter)=proxy_iter{
                            for proxy in proxy_iter{
                                let proxy=proxy?;
                                agent.proxy.push(proxy);
                            }
                        }
                        return Ok(agent)
                    }
                    
                    
                }
            },
            Err(e) => {
                return Err(anyhow!("{:?}",e))
            },
        }
        
        Err(anyhow!(""))
    }

    fn get_proxy(&self,bind_port:u16)->anyhow::Result<(String,Proxy)> {
        // let conn=CONN.lock().unwrap();
        let conn=CONN.try_lock();
        match conn {
            Ok(conn) => {
                if let Some(conn)=conn.as_ref(){
                    let mut stmt = conn.prepare("SELECT agent_id,bind_port,target_host,target_port,enable,whitelist,max_rate from proxy where bind_port=?")?;
                    let proxy_iter = stmt.query_map([bind_port], |row| {
                        let whiltelist:String=row.get(5).unwrap();
                        let ve:Vec<&str>=whiltelist.split(",").collect();
                        let mut v:Vec<String>=vec![];
                        if ve.len()>=1&&ve[0]!=""{
                            v=ve.iter().map(|f|f.to_string()).collect();
                        }
                        Ok((
                            row.get(0).unwrap(),
                            row.get(1).unwrap(),
                            row.get(2).unwrap(),
                            row.get(3).unwrap(),
                            row.get(4).unwrap(),
                            v,
                            row.get(6).unwrap(),
                        ))
                    })?;
                    for proxy in proxy_iter{
                        let proxy=proxy?;
                        return Ok((proxy.0,Proxy{
                            bind_port: proxy.1,
                            target_host: proxy.2,
                            target_port: proxy.3,
                            enable: proxy.4,
                            whitelist: proxy.5,
                            max_rate: proxy.6,
                        }))
                    }
                }
            },
            Err(e) => {
                return Err(anyhow!("{:?}",e));
            },
        }
        
        Err(anyhow!(""))
    }

    fn modify_agent(&self,agent:Agent)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("update agent set name=? where agent_id=?")?;
            stmt.execute((agent.name,agent.id))?;
        }
        Ok(())
    }

    fn get_proxys(&self,agent_id:String)->anyhow::Result<Vec<Proxy>> {
        let conn=CONN.lock().unwrap();
        let mut proxys=vec![];
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("SELECT bind_port,target_host,target_port,enable,whitelist,max_rate from proxy where agent_id=?")?;
            let proxy_iter = stmt.query_map([agent_id], |row| {
                let whiltelist:String=row.get(4).unwrap();
                let ve:Vec<&str>=whiltelist.split(",").collect();
                let mut v:Vec<String>=vec![];
                if ve.len()>=1&&ve[0]!=""{
                    v=ve.iter().map(|f|f.to_string()).collect();
                }

                Ok(Proxy {
                    bind_port: row.get(0).unwrap(),
                    target_host: row.get(1).unwrap(),
                    target_port: row.get(2).unwrap(),
                    enable: row.get(3).unwrap(),
                    whitelist: v,
                    max_rate: row.get(5).unwrap(),
                })
            })?;
            for proxy in proxy_iter{
                let proxy=proxy?;
                proxys.push(proxy);
            }
        }
        Ok(proxys)
    }

    fn modify_proxy(&self,proxy:Proxy)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("update proxy set target_host=?,target_port=?,enable=?,max_rate=? where bind_port=?")?;
            stmt.execute((proxy.target_host,proxy.target_port,proxy.enable,proxy.max_rate,proxy.bind_port))?;
        }
        Ok(())
    }

    fn get_user(&self,username:String)->anyhow::Result<(String,String)> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("SELECT username,password from user where username=?")?;
            let user_iter = stmt.query_map([username], |row| {
                Ok((row.get(0).unwrap(),row.get(1).unwrap()))
            })?;
            for user in user_iter{
                return Ok(user?)
            }
        }
        Err(anyhow!(""))
    }

    fn add_user(&self,username:String,password:String)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("insert into user values(?,?)")?;
            stmt.execute((username,password))?;
        }
        Ok(())
    }

    fn remove_user(&self,username:String)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("delete from user where username=?")?;
            stmt.execute([username])?;
        }
        Ok(())
    }

    fn modify_user(&self,username:String,password:String)->anyhow::Result<()> {
        let conn=CONN.lock().unwrap();
        if let Some(conn)=conn.as_ref(){
            let mut stmt = conn.prepare("update user set password=? where username=?")?;
            stmt.execute((username,password))?;
        }
        Ok(())
    }
}
impl Plugin for SqlitePlugin{

    fn version(&self)->String {
        let version: &'static str = env!("CARGO_PKG_VERSION");
        version.into()
    }

    fn author(&self)->String {
        let author: &'static str = env!("CARGO_PKG_AUTHORS");
        author.into()
    }
    fn info(&self)->String {
        "sqlite database plugin".into()
    }

    fn plugin_type(&self)->east_plugin::plugin::Type {
        Type::DatabasePlugin
    }
    
}
unsafe impl Send for SqlitePlugin{
    
}