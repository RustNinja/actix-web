pub mod State;
#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
#[allow(unused_imports)]
use placex_model::*;
use crate::route::State::AppState;

#[get("/")]
pub async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String{
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[get("/increment_state")]
pub async fn increment_state(data : web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter +=1;
    
    HttpResponse::Ok().body(format!("{counter}"))
}