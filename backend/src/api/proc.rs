use std::fmt::format;

use actix_web::{Error, HttpRequest, HttpResponse, Result,http::header};


pub async fn index(req:HttpRequest) -> Result<HttpResponse, Error>{
    let accept_header = req
    .headers()
    .get(header::ACCEPT)
    .and_then(|value| value.to_str().ok())
    .unwrap_or("");
    match accept_header{
        "text/html" => {

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
        format!("{}",
r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
<h1>hello world</h1>    
</body>
</html>
"#
    )
    ))
        }
    // "application/wasm" => {
    //     Ok(HttpResponse::Ok()
    //         .content_type("application/wasm; charset=utf-8")
    //         .body()
    //     )
    // }
    _ =>{
        Ok(HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(format!("Requested format is not supported. {} is invalid accept_header",accept_header)))
    }
    }
}