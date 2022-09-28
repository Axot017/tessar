mod controller;
mod dto;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api").service(
                web::scope("/v1").service(
                    web::scope("dart")
                        .service(controller::version)
                        .service(controller::analyze)
                        .service(controller::format),
                ),
            ),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
