use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use tera::Context;
use crate::ui::server::AppData;

// #[get("/")]
pub async fn index(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok()
        .body(rendered)
}
