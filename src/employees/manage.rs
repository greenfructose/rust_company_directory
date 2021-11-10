use crate::departments::manage::Department;
use jammdb::{DB, Data, Error};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
// pub fn add(employee: Employee) -> String {
//     department.name
// }
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Employee {
    name: String,
    extension: String,
    department: Department,
}

