// api for user authorization
// curl http://localhost:3000/api/auth_user
use auth0_rs::Auth0;
use tuono_lib::axum::http::StatusCode;
use tuono_lib::Request;
// ..custom
use tuono_tutorial::internal::auth_function::get_jwks;

#[derive(serde::Serialize)]
pub struct AuthResponse {
    status: String,
    msg: String,
}



#[tuono_lib::api(GET)]
pub async fn auth_user(_req: Request) -> StatusCode {    

    let keys = get_jwks().await.expect("Error fetching JWKS");
    let auth0 = Auth0::new(keys.as_str()).expect("Error initializing auth0");

    // TODO - Retrieve from request header
    let access_token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IkFfMUp3eGJYMkZYeVBGVTNiRFlRcSJ9.eyJpc3MiOiJodHRwczovL2Rldi1wMndzZnc2aC51cy5hdXRoMC5jb20vIiwic3ViIjoiQTBaVTlweG15WHdZZWhUZ0p4S3B6UmhrWlN4ellFbDZAY2xpZW50cyIsImF1ZCI6Imh0dHBzOi8vcHJhbW9kLXByb2ZpbGUubmV0IiwiaWF0IjoxNzM3ODUwOTA4LCJleHAiOjE3Mzc5MzczMDgsImd0eSI6ImNsaWVudC1jcmVkZW50aWFscyIsImF6cCI6IkEwWlU5cHhteVh3WWVoVGdKeEtwelJoa1pTeHpZRWw2IiwicGVybWlzc2lvbnMiOltdfQ.YlGGLnjZ7t9uGAtTE1o9EKgUmVS47o1c0_0baHBd8RR5dil1tCNc39D73mj6tmAwrXpzwPZ6LtypcQ-812GALemo87nL2y551PsNA4-NN5vMy9iqbodLNhiRGOquuIBpKROfAoRURZxk7X53UKAydVo-6Pbn_5RtT9nzrsNCvoBJzxtHi56HArdmqlwit5jDP7w73FuTsc6m03fWgh2k6khxKiKoANmghnYF0eFdvfXkL-Z1SsvENnmapzz4BJJLUalY43uR4NJEriBswcRyfjPY9Rl5WqhABwcYOmWHa1xldZfSqqvdk7_XSRCRBWYtcwG3fIw67AHt0NUPel7Ujg";
    match auth0.validate_token(access_token) {
        Ok(claims) => {
            println!("Token is valid. Claims is: {:?}", claims);
            StatusCode::OK
        }
        Err(err) => {
            println!("Token validation failed: {:?}", err);
            StatusCode::UNAUTHORIZED
        }
    }
}
