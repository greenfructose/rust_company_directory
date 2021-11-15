use postgres::{Connection, TlsMode};
use openssl::ssl::{SslConnector, SslConnectorBuilder, SslMethod, SslVerifyMode};
use openssl::x509;
use postgres::error::Error;
use crate::config::manage::get_db_config;

// Get DB connection
pub fn get_db_connection() -> Result<Connection, Error> {
    let mut connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
    let config = get_db_config().unwrap();
    let ca_file = config.get(&"sslrootcert").unwrap().unwrap();
    let cert_file = config.get(&"sslcert").unwrap().unwrap();
    let key_file = config.get(&"sslkey").unwrap().unwrap();
    connector
        .set_ca_file(ca_file)
        .unwrap();
    connector
        .set_certificate_file((cert_file), x509::X509_FILETYPE_PEM)
        .unwrap();
    connector
        .set_private_key_file(key_file, x509::X509_FILETYPE_PEM)
        .unwrap();
    let mode = SslVerifyMode::empty();
    connector.set_verify(mode);
    let negotiator = postgres::tls::openssl::OpenSsl::from(connector.build());
    let connection_string = config.get(&"connection_string").unwrap().unwrap();
    let conn = Connection::connect(
        connection_string,
        TlsMode::Require(&negotiator),
    ).unwrap();
    Ok(conn)
}