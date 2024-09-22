use actix_web::{Error, HttpRequest, HttpResponse, Result};


/// index
pub async fn index(_:HttpRequest) -> Result<HttpResponse, Error>{
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
        format!("{}",
r#"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>This is me</title>
        <link href="./static/output.css" rel="stylesheet">
        <script type="module">
            import init from "./static/wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
"#
    )
    ))
}

