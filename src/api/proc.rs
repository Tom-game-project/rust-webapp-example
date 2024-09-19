use actix_web::{Error, HttpRequest, HttpResponse, Result};

pub async fn index(_:HttpRequest) -> Result<HttpResponse, Error>{
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
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