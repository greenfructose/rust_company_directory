use crate::departments::manage::{Department, DepartmentList};
use crate::employees;
use crate::employees::manage::{Employee, EmployeeList};
use crate::ui::server::AppData;
use crate::users;
use crate::users::manage::{User, UserList};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tera::Context;
use serde::{Deserialize, Serialize};

// #[get("/")]
// pub async fn index(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
//     let mut ctx = Context::new();
//     let rendered = data.tmpl.render("index.html", &ctx).unwrap();
//     HttpResponse::Ok().body(rendered)
// }

#[get("/employees")]
pub async fn index(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let employees = EmployeeList {
        employees: employees::manage::list().unwrap(),
    };
    let departments = DepartmentList {
        departments: departments::manage::list().unwrap(),
    };
    println!("{:?}", employees);
    ctx.insert("employees", &employees);
    ctx.insert("departments", &departments);
    let rendered = data.tmpl.render("employees.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}


pub async fn users(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let users = UserList {
        users: users::manage::list().unwrap(),
    };
    ctx.insert("users", &users);
    let rendered = data.tmpl.render("users.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn departments(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let departments = DepartmentList {
        departments: departments::manage::list().unwrap(),
    };
    ctx.insert("departments", &departments);
    let rendered = data.tmpl.render("departments.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
