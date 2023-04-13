use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{web, Responder, get,HttpRequest, HttpResponse, Error, post};
use east_plugin::control::{AgentControl, ProxyControl};
use east_plugin::{plugin::DatabasePlugin, proxy::Proxy};
use east_plugin::agent::Agent;
use serde::{Serialize, Deserialize};

use crate::model::agent::AgentModel;
use crate::auth;
use crate::user_data::UserData;
use rust_embed::RustEmbed;
use mime_guess::from_path;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => HttpResponse::Ok()
      .content_type(from_path(path).first_or_octet_stream().as_ref())
      .body(content.data.into_owned()),
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}
#[get("/")]
pub async fn index()->impl Responder{
    handle_embedded_file("index.html")
}
#[get("/assets/{_:.*}")]
pub async fn dist(path: web::Path<String>) -> impl Responder {
  handle_embedded_file(format!("assets/{}",path.as_str()).as_str())
}

#[get("/agent")]
pub async fn agents(user: UserData,data: web::Data<Box<dyn DatabasePlugin>>,ac: web::Data<Box<dyn AgentControl>>) -> impl Responder {
    let agents=data.get_agents().unwrap();
    let mut results=vec![];
    for a in agents.iter(){
        results.push(AgentModel{
            id:a.id.clone(),
            name:a.name.clone(),
            is_online:ac.clone().is_online(a.id.clone()),
            proxy:a.proxy.clone(),
        })
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:2000,info:"成功".into(),data:results})
}

#[post("/agent/add")]
pub async fn add_agent(user: UserData,agent:web::Json<Agent>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let result=data.add_agent(agent.clone());
    match result{
        Ok(())=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:2000,info:"成功".into(),data:()}),
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("{:?}",e),data:()})
    }
    
}

#[get("/agent/remove/{id}")]
pub async fn remove_agent(user: UserData,agent:web::Path<String>,data: web::Data<Box<dyn DatabasePlugin>>,ac: web::Data<Box<dyn AgentControl>>) -> impl Responder {
    let id=agent.into_inner();
    let agent_id=id.clone();
    let result=data.remove_agent(id);
    match result{
        Ok(())=>{
            ac.close(agent_id);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(Resp{code:2000,info:"成功".into(),data:()})
        },
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("{:?}",e),data:()})
    }
    
}


#[get("/proxy/{agent_id}")]
pub async fn proxys(user: UserData,agent_id:web::Path<String>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let proxy=data.get_proxys(agent_id.clone()).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:2000,info:"成功".into(),data:proxy})
}

#[post("/proxy/add/{agent_id}")]
pub async fn add_proxy(user: UserData,agent_id:web::Path<String>,proxy:web::Json<Proxy>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let result=data.add_proxy(agent_id.clone(),proxy.clone());
    match result{
        Ok(())=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:2000,info:"成功".into(),data:()}),
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("{:?}",e),data:()})
    }
    
}
#[get("/proxy/remove/{bind_port}")]
pub async fn remove_proxy(user: UserData,bind_port:web::Path<u16>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let bind_port=bind_port.into_inner();
    let result=data.get_proxy(bind_port);
    match result{
        Ok((_,proxy))=>{
            if proxy.enable{
                return HttpResponse::Ok().json(Resp{code:40001,info:"只能移除已禁用的代理转发".into(),data:()})
            }
            let result=data.remove_proxy(bind_port);
            match result{
                Ok(())=>{
                    HttpResponse::Ok()
                .content_type("application/json")
                .json(Resp{code:2000,info:"成功".into(),data:()})},
                Err(e)=>HttpResponse::Ok()
                .content_type("application/json")
                .json(Resp{code:4000,info:format!("{:?}",e),data:()})
            }
        },
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("不存在的代理{:?}",e),data:()})
    }
    
    
}

#[post("/proxy/modify")]
pub async fn modify_proxy(user: UserData,proxy:web::Json<Proxy>,data: web::Data<Box<dyn DatabasePlugin>>,pc: web::Data<Box<dyn ProxyControl>>) -> impl Responder {
    let bind_port=proxy.clone().bind_port;
    let result=data.get_proxy(bind_port);
    match result{
        Ok((agent_id,proxy_old))=>{
            let result=data.modify_proxy(proxy.clone());
            match result{
                Ok(())=>{
                    if proxy_old.enable==true&&proxy.clone().enable==false{
                        pc.stop(agent_id, bind_port);
                    }else if proxy_old.enable==false&&proxy.clone().enable==true{
                        pc.start(agent_id, bind_port);
                    }
                    HttpResponse::Ok()
                .content_type("application/json")
                .json(Resp{code:2000,info:"成功".into(),data:()})
                },
                Err(e)=>HttpResponse::Ok()
                .content_type("application/json")
                .json(Resp{code:4000,info:format!("{:?}",e),data:()})
            }
        },
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("不存在的代理{:?}",e),data:()})
    }
    
    
}

#[derive(Serialize, Deserialize, Debug)]
struct Resp<T>{
    code:u16,
    info:String,
    data:T
}

#[derive(Serialize, Deserialize, Debug)]
struct User{
    pub username:String,
    pub password:String
}


const LOCKED_SECS:u64=1*60*60*3;

const MAX_ERR_COUNT:u64=5;


#[post("/login")]
async fn login(body: web::Json<User>,account: web::Data<(String,String)>,data: web::Data<Box<dyn DatabasePlugin>>,store:web::Data<Mutex<HashMap::<String,u64>>>) -> impl Responder {
    let mut store=store.lock().unwrap();
    
    let username=body.username.clone();
    let result=data.get_user(username.clone());
    
    let locked_key=format!("locked_{}",username.clone());
    let count_key=format!("count_{}",username.clone());
    let secs=SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs=secs.as_secs();
    if let Some(t)=store.get(&locked_key){
        if (secs-t)<LOCKED_SECS{
            return HttpResponse::Ok().json(Resp{code:4006,info:"用户已被锁定".into(),data:()})
        }
        store.remove(&locked_key);
        store.remove(&count_key);
    }
    match result {
        Ok((username,password))=>{
            if password==body.password.clone(){
                store.remove(&locked_key);
                store.remove(&count_key);
                let token = auth::create_jwt(username.clone());
                HttpResponse::Ok()
                .content_type("application/json")
                .json(Resp{code:2000,info:"成功".into(),data:token})
            }else{
                let mut count=1u64;
                if let Some(c)=store.get(&count_key){
                    count=c+1;
                }
                store.insert(count_key,count);
                if count>=MAX_ERR_COUNT{
                    store.insert(locked_key, secs);
                    return HttpResponse::Ok().json(Resp{code:4006,info:"用户已被锁定".into(),data:()})
                }
                
                HttpResponse::Ok()
                .content_type("application/json")
                .json(Resp{code:4000,info:format!("{}密码错误",username),data:()})
            }
            
        },
        Err(e)=>{
            if username==account.0{
                if account.1==body.password.clone(){
                    store.remove(&locked_key);
                    store.remove(&count_key);
                    let token = auth::create_jwt(username.clone());
                    HttpResponse::Ok()
                    .content_type("application/json")
                    .json(Resp{code:2000,info:"成功".into(),data:token})
                }else{
                    let mut count=1u64;
                    if let Some(c)=store.get(&count_key){
                        count=c+1;
                    }
                    store.insert(count_key,count);
                    if count>=MAX_ERR_COUNT{
                        store.insert(locked_key, secs);
                        return HttpResponse::Ok().json(Resp{code:4006,info:"用户已被锁定".into(),data:()})
                    }
                    
                    HttpResponse::Ok()
                    .content_type("application/json")
                    .json(Resp{code:4000,info:format!("{}密码错误",username),data:()})
                }
            }else{
                HttpResponse::Ok()
                .content_type("application/json")
                .json(Resp{code:4000,info:format!("用户不存在{:?}",e),data:()})
            }
        }
    }
    
}


#[post("/login/user")]
async fn login_user_info(user: Option<UserData>) -> impl Responder {
    if let Some(user) = user {
        HttpResponse::Ok().json(Resp{code:2000,info:"获取登录用户成功".into(),data:user.id})
    } else {
        HttpResponse::Ok().json(Resp{code:4000,info:"用户未登录".into(),data:()})
    }
}

