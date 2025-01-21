use reqwest::Client;
use serde::{Deserialize, Serialize};
use libsql::Connection;
use tuono_lib::{Props, Request, Response};

#[derive(Debug, Serialize, Deserialize)]
struct Users {
    results: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    vendor: String,
}

#[tuono_lib::handler]
async fn get_all_users(_req: Request, fetch: Client, sql_conn: Connection) -> Response {
    let mut rows = sql_conn.query("SELECT * FROM users", ()).await.unwrap();
    let mut user_list: Vec<User> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        // Access column values by index or name
        let record: User = User {
            name: row.get(0).unwrap_or("".to_string()),
            email: row.get(1).unwrap_or("".to_string()),
            vendor: row.get(2).unwrap_or("".to_string()),
        };
        user_list.push(record);        
    }  
    let response = Users { results: user_list };
    Response::Props(Props::new(response))    
    
}
