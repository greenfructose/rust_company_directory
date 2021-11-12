use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use tera::Context;
use serde::{Serialize, Deserialize};
use crate::ui::server::AppData;
use crate::employees;
use crate::users;
use crate::departments;
use crate::employees::manage::{Employee, EmployeeList};
use crate::users::manage::{User, UserList};
use crate::departments::manage::{Department, DepartmentList};

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
    ctx.insert("employees", &employees.employees);
    let rendered = data.tmpl.render("employees.html", &ctx).unwrap();
    HttpResponse::Ok()
        .body(rendered)
}

pub async fn users(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let users = UserList {
        users: users::manage::list().unwrap(),
    };
    ctx.insert("users", &users.users);
    let rendered = data.tmpl.render("users.html", &ctx).unwrap();
    HttpResponse::Ok()
        .body(rendered)
}

pub async fn departments(data: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let departments = DepartmentList {
        departments: departments::manage::list().unwrap(),
    };
    ctx.insert("departments", &departments.departments);
    let rendered = data.tmpl.render("departments.html", &ctx).unwrap();
    HttpResponse::Ok()
        .body(rendered)
}
