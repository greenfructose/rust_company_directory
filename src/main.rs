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
    let titles: [String; 6] = ["Boss".to_string(), "Drone".to_string(), "Operator".to_string(), "Driver".to_string(), "Director".to_string(), "Sales".to_string()];
    let depts = departments::manage::list().unwrap();
    let mut reader = csv::Reader::from_path("./src/MOCK_DATA.csv");
    for result in reader.unwrap().records() {
        let record = result.unwrap();
        employees::manage::put(
            record[0].to_string(),
            record[1].to_string(),
            rand::thread_rng().gen_range(1000..10000).to_string(),
            titles[rand::thread_rng().gen_range(0..6)].to_string(),
            depts.choose(&mut rand::thread_rng()).unwrap(),
        );
    };
    webbrowser::open("http://127.0.0.1:8080");
    ui::server::main();
}
