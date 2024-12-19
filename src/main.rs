mod admin;
mod api;

use actix_web::{web, App, HttpServer};
use chrono::Utc;
use chrono_tz::America::Mexico_City;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080 at {}", Utc::now().with_timezone(&Mexico_City));
    HttpServer::new(|| {
        App::new().service(
            web::scope("v1")
                .service(web::scope("/api").configure(api::config))
                .service(web::scope("/admin").configure(admin::config))
        )
    })
    .bind(("127.0.0.1", 8080))?.run().await
}