use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use postgres::Client;
use crate::config::manage::get_db_config;
use std::fs;
use std::error::Error;


// Get DB connection
pub fn get_db_connection() -> Result<Client, Box<dyn Error>> {
    let config = get_db_config();
    let cert_file = config.get("sslrootcert").unwrap().as_ref().unwrap();
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_ca_file(cert_file)?;
    let connector = MakeTlsConnector::new(builder.build());
    let connection_string = config.get("connection_string").unwrap().as_ref().unwrap();
    let client = Client::connect(
        connection_string,
        connector
    ).unwrap();
    Ok(client)
}