
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use postgres::{Client, Error};
use crate::config::manage::get_db_config;
use std::fs;

// Get DB connection
pub fn get_db_connection() -> Result<Client, Error> {
    let config = get_db_config();
    let cert_file = config.get("sslrootcert").unwrap().as_ref().unwrap();
    let cert = fs::read(cert_file).unwrap();
    let cert = Certificate::from_pem(&cert).unwrap();
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build().unwrap();
    let connection_string = config.get("connection_string").unwrap().as_ref().unwrap();
    let connector = MakeTlsConnector::new(connector);
    println!("{}", connection_string);
    let client = Client::connect(
        &*connection_string,
        connector,
    )?;
    Ok(client)
}