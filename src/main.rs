mod departments;
mod employees;
mod ui;
mod users;
mod config;
mod db;
use csv;
use rand;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use tera::helpers::tests::number_args_allowed;
use webbrowser;
use std::iter;
use employees::manage::Employee;
use users::manage::{Role, User};

// use rust_company_directory::departments::manage::Department;
// use rust_company_directory::users::manage::Role::SuperAdmin;

fn main() {
    // generate_mock_data();
    // users::manage::get_it();
    webbrowser::open("http://127.0.0.1:8080");
    ui::server::main();
}

fn generate_mock_data() {
    let titles: [String; 6] = [
        "Boss".to_string(),
        "Drone".to_string(),
        "Operator".to_string(),
        "Driver".to_string(),
        "Director".to_string(),
        "Sales".to_string(),
    ];
    let depts: [String; 6] = [
        "Administration".to_string(),
        "Marketing".to_string(),
        "Sales".to_string(),
        "Production".to_string(),
        "Maintenance".to_string(),
        "I.T.".to_string(),
    ];
    let roles : [String; 3] = [
        "User".to_string(),
        "Admin".to_string(),
        "SuperAdmin".to_string()
    ];
    let mut reader = csv::Reader::from_path("./src/MOCK_DATA.csv");
    let mut employees: Vec<Employee> = Vec::new();
    let mut users: Vec<User> = Vec::new();
    for result in reader.unwrap().records() {
        let mut record = result.unwrap();
        let mut department = depts[thread_rng().gen_range(0..6)].to_string();
        let mut title = titles[thread_rng().gen_range(0..6)].to_string();
        let mut extension = thread_rng().gen_range(1000..10000).to_string();
        let mut first_name = record[0].to_string();
        let mut last_name = record[1].to_string();
        let employee = Employee {
            id: None,
            first_name,
            last_name,
            extension,
            title,
            department
        };

        if employee.department == "I.T.".to_string() {
            let mut rng = thread_rng();
            let mut first_name = record[0].to_string();
            let mut last_name = record[1].to_string();
            let username = String::from(record[0].chars().next().unwrap().to_string() + &record[1])
                    .to_lowercase();
            let password = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(10)
                .collect();
            let role = roles[thread_rng().gen_range(0..3)].to_string();
            let role: Role = role.parse().unwrap();
            let user = User{
                id: None,
                first_name,
                last_name,
                username,
                password: Some(password),
                role
            };
            users.push(user);
        }
        employees.push(employee);
    }
    employees::manage::put(employees);
    users::manage::put(users);
    // println!("Adding mock departments ...");
    // for dept in depts {
    // departments::manage::put(dept.to_string());
    // }
    println!("Done!");
}
