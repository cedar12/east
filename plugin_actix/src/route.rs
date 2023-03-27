use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{web, Responder, get,HttpRequest, HttpResponse, Error, post};
use east_plugin::control::AgentControl;
use east_plugin::{plugin::DatabasePlugin, proxy::Proxy};
use east_plugin::agent::Agent;
use serde::{Serialize, Deserialize};

use crate::auth;
use crate::user_data::UserData;

pub async fn index()->impl Responder{
    format!("east web")
}

#[get("/agent")]
pub async fn agents(user: UserData,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let agents=data.get_agents().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(agents)
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
        .json(proxy)
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
    let result=data.remove_proxy(bind_port);
    match result{
        Ok(())=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:2000,info:"成功".into(),data:()}),
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("{:?}",e),data:()})
    }
    
}

#[post("/proxy/modify")]
pub async fn modify_proxy(user: UserData,proxy:web::Json<Proxy>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let result=data.modify_proxy(proxy.clone());
    match result{
        Ok(())=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:2000,info:"成功".into(),data:()}),
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("{:?}",e),data:()})
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
async fn login(body: web::Json<User>,data: web::Data<Box<dyn DatabasePlugin>>,store:web::Data<Mutex<HashMap::<String,u64>>>) -> impl Responder {
    let result=data.get_user(body.username.clone());
    let mut store=store.lock().unwrap();
    // println!("store->{:?}",store);
    let locked_key=format!("locked_{}",body.username.clone());
    let count_key=format!("count_{}",body.username.clone());
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
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("用户不存在{:?}",e),data:()})
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

