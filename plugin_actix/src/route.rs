use actix_web::{web, Responder, get,HttpRequest, HttpResponse, Error};
use east_plugin::plugin::DatabasePlugin;

pub async fn index()->impl Responder{
    format!("east web")
}

pub async fn agents(data: web::Data<Box<dyn DatabasePlugin>>) -> Result<HttpResponse, Error> {
    let agents=data.get_agents().unwrap();
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(agents))
}
