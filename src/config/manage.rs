use configparser::ini::Ini;
use std::collections::HashMap;
use std::collections;


// Get DB config
pub fn get_db_config() -> HashMap<String, Option<String>> {
    let mut config = Ini::new();
    let map = config.load("./src/config.ini").unwrap();
    map["db"].clone()
}

    