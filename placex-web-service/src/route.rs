pub mod State;
use actix_web::Either;
#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
#[allow(unused_imports)]
use placex_model::*;
use crate::{route::State::AppState, dto::MyDto};

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

async fn ret_my_dto() -> impl Responder {
    MyDto { name: "user" }
}

use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;
async fn Either_index() -> RegisterResult {
    if true {
        // choose Left variant
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        // choose Right variant
        Either::Right(Ok("Hello"))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}