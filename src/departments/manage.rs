use crate::employees::manage::{Employee, EmployeeList};
use crate::db::manage::get_db_connection;
use postgres::{Client, Error};


// Postgres functions
pub fn put(name: String,) -> Result<(), Error> {
    let mut client = get_db_connection().unwrap();
    println!("Create departments table if not exist ... ");
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS department (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL
        )
    ")?;
    let statement = client.prepare("INSERT INTO department (name) VALUES ($1)")?;
    let rows_updated = client.execute(
        &statement,
        &[&name])?;
    println!("Added department: {}, {} rows affected.", name, rows_updated);
    Ok(())
}

pub fn get(id: i32) -> Result<Department, Error> {
    let mut client = get_db_connection().unwrap();
    let mut name: String = "".to_string();
    for row in &client.query("
        SELECT name FROM department WHERE ID = $1
    ", &[&id]).unwrap(){
        name = row.get("name");
    }
    let department = Department{
        id: Some(id),
        name,
    };
    Ok(department)
}



pub async fn list() -> Result<Vec<Department>, Error> {
    let mut client = get_db_connection().unwrap();
    let mut departments = Vec::new();
    for row in &client.query(
        "SELECT id, name FROM department",
        &[])? {
            let id = row.get(0);
            let name= row.get(1);
            let department = Department {
                id: Some(id),
                name,
            };
            departments.push(department);
        }
    {
        Ok(departments)
    }
}

#[derive(Debug, PartialEq)]
pub struct Department {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct DepartmentList {
    pub departments: Vec<Department>,
}
