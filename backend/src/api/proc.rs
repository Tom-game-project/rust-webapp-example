use actix_web::{Error, HttpRequest, HttpResponse, Result};
use std::fs::read_to_string;

/// index
pub async fn index(_:HttpRequest) -> Result<HttpResponse, Error>{
    match read_to_string("../frontend/index.html"){
        Ok(a) => {
            Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(a))
        },
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Failed to read HTML file")),
    }
}

