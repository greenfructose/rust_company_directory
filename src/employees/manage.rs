use crate::departments::manage::Department;
use crate::db::manage::get_db_connection;
use postgres::{Client, Error};

pub fn put(employees: Vec<Employee>) -> Result<(), Error> {
    let mut client = get_db_connection().unwrap();
    println!("Create employee table if not exist ... ");
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS employee (
        id SERIAL PRIMARY KEY,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        extension TEXT NOT NULL,
        title TEXT NOT NULL,
        department TEXT NOT NULL
        )
    ")?;
    let statement = client.prepare("INSERT INTO employee (first_name, last_name, extension, title, department) VALUES ($1, $2, $3, $4, $5)")?;
    let mut rows_updated = 0;
    for employee in employees {
       rows_updated += client.execute(
        &statement,
        &[&employee.first_name, &employee.last_name, &employee.extension, &employee.title, &employee.department])?;
    }
    println!("Added employees, {} rows affected.", rows_updated);
    Ok(())
}

pub fn get(id: i32) -> Result<Employee, Error> {
    let mut client = get_db_connection().unwrap();
    let mut first_name= String::new();
    let mut last_name= String::new();
    let mut extension= String::new();
    let mut title= String::new();
    let mut department= String::new();
    for row in &client.query("
        SELECT first_name, last_name,  extension, title, department FROM employee WHERE ID = $1
    ", &[&id]).unwrap(){
        first_name = row.get("first_name");
        last_name = row.get("last_name");
        extension = row.get("extension");
        title = row.get("title");
        department = row.get("department");
    }
    let id = Some(id);
    let employee = Employee {
        id,
        first_name,
        last_name,
        extension,
        title,
        department,
    };
    Ok(employee)
}

pub async fn list() -> Result<Vec<Employee>, Error> {
    let mut client = get_db_connection().unwrap();
    let mut employees = Vec::new();
    for row in &client.query(
    "SELECT * FROM employee",
    &[])? {
        let id = row.get("id");
        let first_name= row.get("first_name");
        let last_name = row.get("last_name");
        let extension = row.get("extension");
        let title = row.get("title");
        let department = row.get("department");
        let employee = Employee {
            id: Some(id),
            first_name,
            last_name,
            extension,
            title,
            department
        };
        employees.push(employee);
    }
    Ok(employees)
}


#[derive(Debug, PartialEq)]
pub struct Employee {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub extension: String,
    pub title: String,
    pub department: String,
}

#[derive(Debug, PartialEq)]
pub struct EmployeeList {
    pub employees: Vec<Employee>,
}
