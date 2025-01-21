mod internal;

use internal::db_config;
use reqwest::Client;
use libsql::Connection;
// Import here the just added reqwest library

#[derive(Clone)]
// Extend this struct with the feature you will need for your application
pub struct ApplicationState {
    // This will be available to all your route handlers
    pub fetch: Client,
    pub sql_conn: Connection,
}

pub fn main() -> ApplicationState {
    let fetch = Client::new();    
    return ApplicationState {
        fetch,
        sql_conn: db_config::init_connection(),
    };
}
