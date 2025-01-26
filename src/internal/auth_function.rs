/**
 * Auth Function
 * checks JWKS and checks if token is valid
 */
use anyhow::Error;
use reqwest::Client;
use dotenvy::dotenv;

pub async fn get_jwks() -> Result<String, Error> {
    dotenv().ok();
    let fetch = Client::new();

    let jwks_url = std::env::var("AUTH0_JWKS_URL")?;
    println!("JWKS URL is: {}", &jwks_url);
    match fetch.get(jwks_url).send().await {
        Ok(res) => {
            match res.json::<String>().await {
                Ok(data) => {
                    Ok(data)
                }
                Err(err) => {
                    let err_msg = format!("Error parsing JWKS: {:?}", err);
                    Err(Error::msg(err_msg))
                },
            }             
        }
        Err(err) => {
            let err_msg = format!("Error fetching JWKS: {:?}", err);
            Err(Error::msg(err_msg))            
        },
    }    
}