
use actix_web::{get, web, App, HttpServer, Responder, rt::Runtime};
use route::greet;



#[test]
fn test(){
    let rt = Runtime::new().unwrap();
    let run=HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080)).unwrap()
    .run();
    rt.block_on(run).unwrap();
}