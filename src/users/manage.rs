use crate::employees::manage::Employee;
use crate::db::manage::get_db_connection;
use postgres::{Client, Error};
use pwhash::bcrypt;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use postgres_types::{ToSql, FromSql};
use std::{fmt, iter};
use std::fmt::Formatter;
use std::str::FromStr;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn put(users: Vec<User>) -> Result<(), Error> {
    let mut client = get_db_connection().unwrap();
    println!("Create user table if not exist ... ");
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        username TEXT NOT NULL,
        password TEXT NOT NULL,
        role TEXT NOT NULL
        )
    ")?;
    let statement = client.prepare("INSERT INTO users (first_name, last_name, username, password, role) VALUES ($1, $2, $3, $4, $5)")?;
    let mut rows_updated = 0;
    for user in users {
        let hash = bcrypt::hash(String::from(&user.password.unwrap())).unwrap();
        rows_updated += client.execute(
        &statement,
        &[&user.first_name, &user.last_name, &user.username, &hash, &user.role.to_string()])?;
    }
    println!("Added users, {} rows affected.", rows_updated);
    Ok(())
}

pub fn get(id: i32) -> Result<User, Error> {
    let mut client = get_db_connection().unwrap();
    let mut first_name= String::new();
    let mut last_name= String::new();
    let mut username= String::new();
    let mut role = String::new();
    for row in &client.query("
        SELECT first_name, last_name,  username, role FROM users WHERE ID = $1
    ", &[&id]).unwrap(){
        first_name = row.get("first_name");
        last_name = row.get("last_name");
        username = row.get("username");
        role = row.get("role");
    }
    let role: Role = role.parse().unwrap();
    let id = Some(id);
    let user = User {
        id,
        first_name,
        last_name,
        username,
        password: None,
        role,
    };
    Ok(user)
}
pub async fn list() -> Result<Vec<User>, Error> {
    let mut client = get_db_connection().unwrap();
    let mut users = Vec::new();
    for row in &client.query(
        "SELECT id, first_name, last_name, username, role FROM users",
        &[])? {
            let id = row.get("id");
            let first_name= row.get("first_name");
            let last_name = row.get("last_name");
            let username = row.get("username");
            let role: String = row.get("role");
            let role: Role = role.parse().unwrap();
            let user = User {
                id: Some(id),
                first_name,
                last_name,
                username,
                password: None,
                role
            };
            users.push(user);
        }
        Ok(users)
}

pub fn get_it() -> Result<(), Error> {
    let mut client = get_db_connection().unwrap();
    let mut users: Vec<User> = Vec::new();
    let roles : [String; 3] = [
        "User".to_string(),
        "Admin".to_string(),
        "SuperAdmin".to_string()
    ];
    for row in &client.query(
        "SELECT first_name, last_name, department FROM employee WHERE department = 'I.T.'",
        &[]
    )?{
        let mut rng = thread_rng();
        let first_name: String = row.get("first_name");
        let last_name: String = row.get("last_name");
        let username = String::from(first_name.chars().next().unwrap().to_string() + &last_name)
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
    put(users);
    Ok(())
}

#[derive(Debug, ToSql, FromSql, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: Option<String>,
    pub role: Role,
}
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub(crate) struct UserList {
    pub users: Vec<User>,
}
#[derive(Debug, ToSql, FromSql, PartialEq, Deserialize, Serialize)]
pub enum Role {
    User,
    Admin,
    SuperAdmin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Role {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "User" => Ok(Role::User),
            "Admin" => Ok(Role::Admin),
            "SuperAdmin" => Ok(Role::SuperAdmin),
            _ => Err(format!("'{}' is not a valid value for Role", s)),
        }
    }
}