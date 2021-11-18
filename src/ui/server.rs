use crate::ui::apps::{employees, departments, users, index};
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use actix_web::middleware::Logger;
use env_logger::{Env, Builder};
use tera::Tera;
// use crate::ui::middleware::{extract};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/ui/templates/**/*"
        ))
        .unwrap();
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // .wrap(GrantsMiddleware::with_extractor(extract))
            .data(AppData { tmpl: tera })
            .service(fs::Files::new("/static", "./src/ui/static").show_files_listing())
            // .service(
            //     web::scope("/").route("", web::get().to(index)))
            .service(
                web::scope("/").route("employees", web::get().to(index)))
            .service(
                web::scope("/").route("departments",web::get().to(departments)))
            .service(
                web::scope("/").route("users",web::get().to(users)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub struct AppData {
    pub tmpl: Tera,
}
