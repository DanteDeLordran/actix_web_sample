use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?.run().await
}

#[get("/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("message")
}