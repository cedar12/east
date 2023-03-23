use actix_web::{web, Responder, get,HttpRequest, HttpResponse, Error, post};
use east_plugin::{plugin::DatabasePlugin, proxy::Proxy};
use east_plugin::agent::Agent;
use serde::{Serialize, Deserialize};

pub async fn index()->impl Responder{
    format!("east web")
}

#[get("/agent")]
pub async fn agents(data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let agents=data.get_agents().unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(agents)
}

#[post("/agent/add")]
pub async fn add_agent(agent:web::Json<Agent>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
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
pub async fn remove_agent(agent:web::Path<String>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
    let id=agent.into_inner();
    let result=data.remove_agent(id);
    match result{
        Ok(())=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:2000,info:"成功".into(),data:()}),
        Err(e)=>HttpResponse::Ok()
        .content_type("application/json")
        .json(Resp{code:4000,info:format!("{:?}",e),data:()})
    }
    
}

#[post("/proxy/add/{agent_id}")]
pub async fn add_proxy(agent_id:web::Path<String>,proxy:web::Json<Proxy>,data: web::Data<Box<dyn DatabasePlugin>>) -> impl Responder {
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

#[derive(Serialize, Deserialize, Debug)]
struct Resp<T>{
    code:u16,
    info:String,
    data:T
}