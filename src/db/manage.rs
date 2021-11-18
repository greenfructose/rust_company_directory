use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode, SslFiletype};
use postgres_openssl::MakeTlsConnector;
use postgres::Client;
use crate::config::manage::get_db_config;
use std::fs;
use std::error::Error;


// Get DB connection
pub fn get_db_connection() -> Result<Client, Box<dyn Error>> {
    let config = get_db_config();
    let root_crt_file = config.get("root_crt").unwrap().as_ref().unwrap();
    let client_crt = config.get("client_crt").unwrap().as_ref().unwrap();
    let key_file = config.get("key_file").unwrap().as_ref().unwrap();
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    builder.set_ca_file(root_crt_file)?;
    builder.set_certificate_file(client_crt, SslFiletype::PEM)?;
    builder.set_private_key_file(key_file, SslFiletype::PEM)?;
    let connector = MakeTlsConnector::new(builder.build());
    let connection_string = config.get("connection_string").unwrap().as_ref().unwrap();
    let client = Client::connect(
        connection_string,
        connector
    ).unwrap();
    Ok(client)
}