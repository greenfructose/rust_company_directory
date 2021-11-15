use configparser::ini::Ini;
use std::collections::HashMap;
use postgres::error::Error;

// Get DB config
pub fn get_db_config() -> Result<(HashMap<String, Option<String>>), Error> {
    let mut config = Ini::new();
    let map = config.load("./src/config.ini")?;
    {
    Ok(map["DB"].clone())
    }
}

    