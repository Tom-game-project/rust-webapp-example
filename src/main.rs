use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod api;
use api::proc::index;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let mut port = 8080;
    HttpServer::new(move || {
        let mut cors = Cors::default();
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            // .app_data(tow_truck_service.clone())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/main")
                            .route(web::get().to(index)),
                    )
            )
            .service(
                web::resource("/servise")
                    .route(web::get().to(index)),
            )
    })
    .bind(format!("0.0.0.0:{port}"))?
    .workers(1)
    .run()
    .await
}
