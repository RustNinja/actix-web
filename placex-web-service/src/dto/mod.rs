use actix_web::{Responder, body::BoxBody, HttpRequest, HttpResponse, http::header::ContentType};
use serde::Serialize;

#[derive(Serialize)]
pub struct MyDto{
    pub name : &'static str,
}

impl Responder for MyDto {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}