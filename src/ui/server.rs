use actix_web::{HttpServer, web, App};
use actix_files as fs;
use actix_web_grants::GrantsMiddleware;
use tera::Tera;
use crate::ui::apps::{index, employees, departments};
// use crate::ui::middleware::{extract};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/ui/templates/**/*")
        ).unwrap();
        App::new()
            // .wrap(GrantsMiddleware::with_extractor(extract))
            .data(AppData{tmpl: tera })
            .service(fs::Files::new("/static", "./src/ui/static").show_files_listing())
            .service(web::resource("/")
                .route(web::get().to(index)))
            .service(web::resource("/employees")
                .route(web::get().to(employees)))
            .service(web::resource("/departments")
                .route(web::get().to(departments)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub struct AppData {
    pub tmpl: Tera,
}