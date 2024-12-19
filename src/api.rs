use actix_web::{get, web, HttpResponse};
use actix_web::http::header;
use actix_web::http::header::ContentType;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users).service(get_notes);
}

#[get("/users")]
async fn get_users() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(header::ContentType::json())
        .body(
            r#"
        [
            {
                "id": 1,
                "nombre": "Rusty"
            },
            {
                "id": 2,
                "nombre": "Full Stack"
            }
        ]
        "#,
        )
}

#[get("/notes")]
async fn get_notes() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        r#"
        [
            {
                "id": 1,
                "contenido": "Nota 1"
            },
            {
                "id": 2,
                "contenido": "Nota 2"
            }
        ]
        "#,
    )
}
