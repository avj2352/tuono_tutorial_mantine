use libsql::{Connection, Database};
use dotenvy::dotenv;


pub fn init_connection() -> Connection {
    dotenv().ok();

    let db_url = std::env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let auth_token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    let db = Database::open_remote(db_url, auth_token).unwrap();        

    let conn: Connection = db.connect().unwrap();
    conn
}
