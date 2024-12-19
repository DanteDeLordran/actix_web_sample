use actix_web::{get, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(login)
        .service(init);
}

#[get("/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login Form")
}

#[get("/init")]
async fn init() -> HttpResponse {
    HttpResponse::Ok().body("Hola Admin!")
}