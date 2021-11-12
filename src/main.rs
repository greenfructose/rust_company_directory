mod departments;
mod employees;
mod ui;
mod users;
use webbrowser;
use rand;
use csv;
use rand::Rng;
use rand::seq::SliceRandom;
use tera::helpers::tests::number_args_allowed;


// use rust_company_directory::departments::manage::Department;
// use rust_company_directory::users::manage::Role::SuperAdmin;

fn main() {
    generate_mock_data();
    webbrowser::open("http://127.0.0.1:8080");
    ui::server::main();
}

fn generate_mock_data() {
    let titles: [String; 6] = ["Boss".to_string(), "Drone".to_string(), "Operator".to_string(), "Driver".to_string(), "Director".to_string(), "Sales".to_string()];
    let depts:[String; 6] = ["Administration".to_string(), "Marketing".to_string(), "Sales".to_string(), "Production".to_string(), "Maintenance".to_string(), "I.T.".to_string()];
    let mut reader = csv::Reader::from_path("./src/MOCK_DATA.csv");
    let mut x = 0;
    for result in reader.unwrap().records() {
        let record = result.unwrap();
        employees::manage::put(
            record[0].to_string(),
            record[1].to_string(),
            rand::thread_rng().gen_range(1000..10000).to_string(),
            titles[rand::thread_rng().gen_range(0..6)].to_string(),
            depts[rand::thread_rng().gen_range(0..6)].to_string(),
        );
        if x > 10 {
            users::manage::put(
                record[0].to_string(),
                record[1].to_string(),
                String::from(record[0].chars().next().unwrap().to_string() + &record[1]).to_lowercase(),
                String::from("Password"),
                    users::manage::Role::SuperAdmin,
            )
        }
        x += 1;
    }
    for dept in depts{
        departments::manage::put(dept.to_string(),
        None,
        );
    }
}