use crate::departments::manage::Department;
use jammdb::{DB, Data, Error};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

pub fn put(first_name: String, last_name: String, extension: String, title: String, department: &Department) -> Result<(), Error>{
    {
        let db = DB::open("database.db")?;
        let mut tx = db.tx(true)?;
        let bucket = tx.get_or_create_bucket("employees")?;
        let id = bucket.next_int();
        let mut employee = Employee {
            id,
            first_name,
            last_name,
            extension,
            title,
            department,
        };
        let bytes = rmp_serde::to_vec(&employee).unwrap();
        bucket.put(id.to_le_bytes(), bytes)?;
        tx.commit()?;
    }
    {
        Ok(())
    }
}

pub fn get(id: u64) -> Result<Employee<'static>, Error> {
    let db = DB::open("database.db")?;
    let mut tx = db.tx(false)?;
    let bucket = tx.get_bucket("employees")?;
    match bucket.get(&id.to_le_bytes()){
        Some(data) => {
            match &*data {
                Data::KeyValue(kv) => {
                    let employee: Employee = rmp_serde::from_slice(kv.value()).unwrap();
                    Ok(employee)
                }
                _ => Err(Error::KeyValueMissing)
            }
        }
        None => Err(Error::KeyValueMissing)
    }
}

pub fn list() -> Result<Vec<Employee<'static>>, Error>{
        let db = DB::open("database.db")?;
        let mut tx = db.tx(false)?;
        let bucket = tx.get_bucket("employees")?;
        let mut employees = Vec::new();
        for data in bucket.cursor(){
            if data.is_kv() {
                let kv = data.kv();
                let employee: Employee = rmp_serde::from_slice(kv.value()).unwrap();
                employees.push(employee);
            }
        }
    {
        Ok((employees))
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Employee<'a> {
    id: u64,
    first_name: String,
    last_name: String,
    extension: String,
    title: String,
    department: &'a Department<'a>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EmployeeList<'a> {
    pub(crate) employees: Vec<Employee<'a>>,
}