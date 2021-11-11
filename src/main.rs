mod departments;
mod employees;
mod ui;
mod users;
use webbrowser;
// use rust_company_directory::users::manage::Role::SuperAdmin;

fn main() {
    // departments::manage::add(String::from("Accounting"), None);
    // let name = String::from("Donnie");
    // let username = String::from("baker");
    // let password = String::from("derper");
    // let role = users::manage::Role::SuperAdmin;
    // users::manage::add(name, username, password, role);
    // users::manage::list();
    users::manage::get(0);
    webbrowser::open("http://127.0.0.1:8080");
    ui::server::main();
}
