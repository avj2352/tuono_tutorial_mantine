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
    let mut rows = sql_conn.query("SELECT * FROM users", ()).await.expect("Failed to execute query");
    let mut user_list: Vec<User> = Vec::new();
    while let Some(row) = rows.next().expect("Failed to fetch row") {
        // println!("->> Row is: {:?}", row);                
        // dbg!(&row);
           let user = User {
                name: if let Some(val) = row.get_value(1).unwrap().as_text() {
                    val.to_string()
                } else {
                    "".to_string()
                } ,
                email: if let Some(val) = row.get_value(2).unwrap().as_text() {
                    val.to_string()
                } else {
                    "".to_string()
                },
                vendor: if let Some(val) = row.get_value(3).unwrap().as_text() {
                    val.to_string()
                } else {
                    "".to_string()
                },
            };
            user_list.push(user);       
    }  
    let response = Users { results: user_list };
    Response::Props(Props::new(response))    
    
}
