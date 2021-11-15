use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use crate::config::manage::get_db_config;

// Get DB connection
pub fn get_db_connection() -> Result<PgConnection, sqlx::error> {
    let cert = get_db_config().unwrap().get("sslrootcer").unwrap();
}