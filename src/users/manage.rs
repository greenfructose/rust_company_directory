use actix_web::test::ok_service;
use jammdb::{DB, Data, Error};
use jammdb::Error::KeyValueMissing;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use pwhash::bcrypt;

pub fn add(name: String, username: String, password: String, role: Role) -> Result<(), Error>{
    {
        let db = DB::open("database.db")?;
        let mut tx = db.tx(true)?;
        let user_bucket = tx.get_or_create_bucket("users")?;
        let id = user_bucket.next_int();
        let mut user = User {
            id,
            name,
            username,
            password,
            role
        };
        let h = bcrypt::hash(String::from(&user.password)).unwrap();
        user.password = String::from(h);
        let user_bytes = rmp_serde::to_vec(&user).unwrap();
        user_bucket.put(id.to_le_bytes(), user_bytes)?;
        tx.commit()?;
    }
    {
        Ok(())
    }
}

pub fn get(id: u64) -> Result<User, Error> {
    let db = DB::open("database.db")?;
    let mut tx = db.tx(false)?;
    let bucket = tx.get_bucket("users")?;
    match bucket.get(&id.to_le_bytes()){
        Some(data) => {
            match &*data {
                Data::KeyValue(kv) => {
                    let user: User = rmp_serde::from_slice(kv.value()).unwrap();
                    Ok(user)
                }
                _ => Err(Error::KeyValueMissing)
            }
        }
        None => Err(Error::KeyValueMissing)
    }
    // let kv = bucket.get_kv(&id.to_le_bytes()).unwrap();
    // let user: User = rmp_serde::from_slice(kv.value()).unwrap();
    // match user {
    //     None => Err(Error::KeyValueMissing),
    //     _ => Ok(user),
    // }
}

pub fn list() -> Result<Vec<User>, Error>{
        let db = DB::open("database.db")?;
        let mut tx = db.tx(false)?;
        let bucket = tx.get_bucket("users")?;
        let mut users = Vec::new();
        for data in bucket.cursor(){
            if data.is_kv() {
                let kv = data.kv();
                let user: User = rmp_serde::from_slice(kv.value()).unwrap();
                users.push(user);
            }
        }
    {
        Ok((users))
    }
}
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    id: u64,
    name: String,
    username: String,
    password: String,
    role: Role,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Role {
    User,
    Admin,
    SuperAdmin,
}