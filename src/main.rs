mod departments;
mod employees;
mod ui;
mod users;
use webbrowser;
use rand;
use csv;
use rand::Rng;
use rand::seq::SliceRandom;

// use rust_company_directory::departments::manage::Department;
// use rust_company_directory::users::manage::Role::SuperAdmin;

fn main() {
    webbrowser::open("http://127.0.0.1:8080");
    ui::server::main();
}
