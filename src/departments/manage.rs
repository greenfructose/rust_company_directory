// bookmark https://crates.io/crates/jammdb

use crate::employees::manage::{Employee, EmployeeList};
use crate::db::manage::get_db_connection;
use jammdb::{Data, Error, DB};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use postgres::Client;


// Postgres functions
pub fn put(name: String,) -> Result<(), Error> {
    let mut conn = get_db_connection().unwrap();
    let mut transaction = conn.transaction().unwrap();
    transaction.batch_execute("
        CREATE TABLE [IF NOT EXISTS] department (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL
        )
    ")?;
    transaction.execute("
        INSERT INTO department (name) VALUES ($1)
    ", &[&name])?;
    transaction.commit().unwrap();
    Ok(())
}

pub fn get(id: i32) -> Result<Department, Error> {
    let mut conn = get_db_connection().unwrap();
    let mut name: String = "".to_string();
    for row in &conn.query("
        SELECT name FROM department WHERE ID = $1
    ", &[&id]).unwrap(){
        name = row.get("name");
    }
    let department = Department{
        id,
        name,
    };
    Ok(department)
}

// jammdb functions

// pub fn put(name: String, employees: Option<EmployeeList>) -> Result<(), Error> {
//     {
//         let db = DB::open("database.db")?;
//         let mut tx = db.tx(true)?;
//         let bucket = tx.get_or_create_bucket("departments")?;
//         let id = bucket.next_int();
//         let mut department = Department {
//             id,
//             name,
//             employees,
//         };
//         let bytes = rmp_serde::to_vec(&department).unwrap();
//         bucket.put(id.to_le_bytes(), bytes)?;
//         tx.commit()?;
//         println!("Added department: {}", department.name);
//     }
//     {
//         Ok(())
//     }
// }

// pub fn get(id: u64) -> Result<Department, Error> {
//     let db = DB::open("database.db")?;
//     let mut tx = db.tx(false)?;
//     let bucket = tx.get_bucket("departments")?;
//     match bucket.get(&id.to_le_bytes()) {
//         Some(data) => match &*data {
//             Data::KeyValue(kv) => {
//                 let department: Department = rmp_serde::from_slice(kv.value()).unwrap();
//                 Ok(department)
//             }
//             _ => Err(Error::KeyValueMissing),
//         },
//         None => Err(Error::KeyValueMissing),
//     }
// }

pub fn list() -> Result<Vec<Department>, Error> {
    let db = DB::open("database.db")?;
    let mut tx = db.tx(false)?;
    let bucket = tx.get_bucket("departments")?;
    let mut departments = Vec::new();
    for data in bucket.cursor() {
        if data.is_kv() {
            let kv = data.kv();
            let department: Department = rmp_serde::from_slice(kv.value()).unwrap();
            departments.push(department);
        }
    }
    {
        Ok((departments))
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Department {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DepartmentList {
    pub(crate) departments: Vec<Department>,
}
