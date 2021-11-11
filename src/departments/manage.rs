// bookmark https://crates.io/crates/jammdb

use jammdb::{DB, Data, Error};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use crate::employees::manage::{Employee, EmployeeList};

pub fn put(name: String, employees: EmployeeList) -> Result<(), Error>{
    {
        let db = DB::open("database.db")?;
        let mut tx = db.tx(true)?;
        let bucket = tx.get_or_create_bucket("departments")?;
        let id = bucket.next_int();
        let mut department = Department {
            id,
            name,
            employees,
        };
        let bytes = rmp_serde::to_vec(&department).unwrap();
        bucket.put(id.to_le_bytes(), bytes)?;
        tx.commit()?;
    }
    {
        Ok(())
    }
}

pub fn get(id: u64) -> Result<Department, Error> {
    let db = DB::open("database.db")?;
    let mut tx = db.tx(false)?;
    let bucket = tx.get_bucket("departments")?;
    match bucket.get(&id.to_le_bytes()){
        Some(data) => {
            match &*data {
                Data::KeyValue(kv) => {
                    let department: Department = rmp_serde::from_slice(kv.value()).unwrap();
                    Ok(department)
                }
                _ => Err(Error::KeyValueMissing)
            }
        }
        None => Err(Error::KeyValueMissing)
    }
}

pub fn list() -> Result<Vec<Department>, Error>{
        let db = DB::open("database.db")?;
        let mut tx = db.tx(false)?;
        let bucket = tx.get_bucket("departments")?;
        let mut departments = Vec::new();
        for data in bucket.cursor(){
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
    id: u64,
    name: String,
    employees: EmployeeList,
}