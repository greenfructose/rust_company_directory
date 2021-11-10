mod departments;
mod employees;
mod ui;
use webbrowser;

fn main() {
    // departments::manage::add(String::from("Accounting"), None);
    webbrowser::open("http://127.0.0.1:8080");
    ui::server::main();
}
