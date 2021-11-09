// bookmark https://crates.io/crates/jammdb

use jammdb::{DB, Data, Error};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use crate::employees::manage::Employee;

pub fn add(name: String, employees: Option<Vec<Employee>>) -> Result<(), Error>{
    let dept = Department {
        name,
        employees,
    };
    {
        let db = DB::open("database.db")?;
        let mut tx = db.tx(true)?;
        let dept_bucket = tx.create_bucket("departments")?;
        let dept_bytes = rmp_serde::to_vec(&dept).unwrap()?;
        dept_bucket.put(b"dept", dept_bytes)?;
        tx.commit()?;
    }
    {
        let db = DB::open("database.db")?;
        let mut tx = db.tx(true)?;
        let dept_bucket = tx.get_bucket("departments")?;
        if let Some(data) = dept_bucket.get(b"dept"){
            if data.is_kv() {
                let kv = data.kv();
                let db_dept: Department = rmp_serde::from_slice(kv.value()).unwrap();
                assert_eq!(db_dept, dept);
            }
        }
    }
    {
        Ok(())
    }
}
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Department {
    name: String,
    employees: Option<Vec<Employee>>,
}