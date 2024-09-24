use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_files::Files;

mod api;
use api::proc::index;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let port = 8000;

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
            .service(Files::new(
                    "/static/",
                    "../frontend/dist/",
                )
                .show_files_listing()
            )
            .service(
                web::resource("/")
                    .route(web::get().to(index)),
            )
    })
    .bind(format!("0.0.0.0:{port}"))?
    .workers(1)
    .run()
    .await
}
