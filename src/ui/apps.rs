use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use tera::Context;
use serde::{Serialize, Deserialize};
use crate::ui::server::AppData;
use crate::employees;
use crate::employees::manage::{Employee, EmployeeList};

// #[get("/")]
pub async fn index(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok()
        .body(rendered)
}

pub async fn employees(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let employees = EmployeeList {
        employees: employees::manage::list().unwrap(),
    };
    ctx.insert("users", &users.users);
    println!("Users Context is: {:?}", ctx.get("users"));
    let rendered = data.tmpl.render("employees.html", &ctx).unwrap();
    HttpResponse::Ok()
        .body(rendered)
}

pub async fn departments(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let rendered = data.tmpl.render("departments.html", &ctx).unwrap();
    HttpResponse::Ok()
        .body(rendered)
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct UserList {
    users: Vec<users::manage::User>,
}