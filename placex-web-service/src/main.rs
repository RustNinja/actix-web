use std::sync::Mutex;

#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use placex_web_service::route::{*,State::{AppState}};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .configure(config)
        .app_data(web::Data::new(AppState{
            app_name : String::from("PlaceX Service"),
            counter : Mutex::new(0),
        }))
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .service(increment_state)
        .service(web::scope("/app").route("/x", web::get().to(manual_hello) ))
        
    })
    .keep_alive(std::time::Duration::from_secs(75))
    .workers(4)
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
