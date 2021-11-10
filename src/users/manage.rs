use actix_web::test::ok_service;
use jammdb::{DB, Data, Error};
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
        user_bucket.put(b"user", user_bytes)?;
        tx.commit()?;
    }
    {
        Ok(())
    }
}

// pub fn get(id: u64){
//     let db = DB::open("database.db")?;
//     let mut tx = db.tx(false)?;
//     let bucket = tx.get_bucket("users")?;
//     let id_string = String::from(id);
//     if let Some(kv) = bucket.get_kv("id") {
//         assert_eq!(kv.value(), b(id_string));
//     }
//
// }

pub fn list() -> Result<(), Error>{
        let db = DB::open("database.db")?;
        let mut tx = db.tx(false)?;
        let user_bucket = tx.get_bucket("users")?;
        for data in user_bucket.cursor(){
            if data.is_kv() {
                let kv = data.kv();
                let db_user: User = rmp_serde::from_slice(kv.value()).unwrap();
                println!("Found {:?}", db_user);
                assert!(bcrypt::verify("password", &db_user.password))
            }
        }
    {
        Ok(())
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