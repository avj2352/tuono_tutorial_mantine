// api for user authorization
// curl http://localhost:3000/api/auth_user
use auth0_rs::Auth0;
use dotenvy::dotenv;
use tuono_lib::axum::http::StatusCode;
use tuono_lib::Request;
use crate::internal::auth_function::get_jwks;


#[derive(serde::Serialize)]
pub struct AuthResponse {
    status: String,
    msg: String,
}



#[tuono_lib::api(GET)]
pub async fn auth_user(_req: Request) -> StatusCode {
    dotenv().ok();

    let keys = r#"
{
    "keys": [
        {
            "kty": "RSA",
            "use": "sig",
            "n": "vLiJJnHtwB2PMmyyZRZsuFE7hiAo8x6dO5rlq9N2mDPwB0VnfAfTfkq8-c95byA_qMYUfWXQv7bets_UGQm029pvf9XccSlvhJ0Nh1mEN-lIk75ivdbMN-yLQgNh4dWjvd5YacQOQ22gUBCLfSqvCcKcntvJTZy9eeLZV5ICP6hJuADipv25tz2kUw53BkUTJYJWAQiw7hki0KUyIkjf97P1qLJXUWV6vz_ohewGt0eM4yTA25ASwyUT9RaadcCX9m3eEhYY6fFN6bT1U8qcFm9rqWpZHb1lAjs71_DvaxkG-MC5gOoGiOkkWeVNIJUKF_Y3-DdAPqWw0EGgGhXh_Q",
            "e": "AQAB",
            "kid": "A_1JwxbX2FXyPFU3bDYQq",
            "x5t": "J7nfr91jktBzJINRk6cNKjS1oHo",
            "x5c": [
                "MIIDDTCCAfWgAwIBAgIJA5vBIgrN6is4MA0GCSqGSIb3DQEBCwUAMCQxIjAgBgNVBAMTGWRldi1wMndzZnc2aC51cy5hdXRoMC5jb20wHhcNMjIwMTA2MDUwMDM4WhcNMzUwOTE1MDUwMDM4WjAkMSIwIAYDVQQDExlkZXYtcDJ3c2Z3NmgudXMuYXV0aDAuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvLiJJnHtwB2PMmyyZRZsuFE7hiAo8x6dO5rlq9N2mDPwB0VnfAfTfkq8+c95byA/qMYUfWXQv7bets/UGQm029pvf9XccSlvhJ0Nh1mEN+lIk75ivdbMN+yLQgNh4dWjvd5YacQOQ22gUBCLfSqvCcKcntvJTZy9eeLZV5ICP6hJuADipv25tz2kUw53BkUTJYJWAQiw7hki0KUyIkjf97P1qLJXUWV6vz/ohewGt0eM4yTA25ASwyUT9RaadcCX9m3eEhYY6fFN6bT1U8qcFm9rqWpZHb1lAjs71/DvaxkG+MC5gOoGiOkkWeVNIJUKF/Y3+DdAPqWw0EGgGhXh/QIDAQABo0IwQDAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBRfSyp9axwlKsItNxeaUP1MD6jhGzAOBgNVHQ8BAf8EBAMCAoQwDQYJKoZIhvcNAQELBQADggEBAEHUVBKdPWz/paLd7mt5V/NCquqw5LEB+xr6I6EZ3wgOItghLj5mVGCGp/U0Q3QxoJA2TYo0stCgovIwNl/zKIWLmoQmh/zuHKh9EckxTdUVjBE66zjrQMCxexdu6JVF1HJwUV/dRni1DaZJyP1dsnGRCMtz0GXqC2sx3PF6a6PqMYwAFo4B46RFoRm7vc4xfRThWTeeS/dfpqAhfkWuJjk8aQYku0eWFHWqt4tdRSMN3FWQNeRIYYal321sSZqBSs0d5xQaHiPP+xPCGOgjD52Zyo35b/baVwAngmEBULPdq4O7jyb5jN4Q1kh15PRkiCyS/UR1BxchFOipDfOwCJE="
            ],
            "alg": "RS256"
        },
        {
            "kty": "RSA",
            "use": "sig",
            "n": "yFYEbpXjauy3jhQl2h7I_7nJE3ofmSA63TuqRBk_u-Ea446cFV8S-hMmHDzytEild3givHbnW3WfCmBX5iuoPi1BzrBBNba3rnrKB2ni2gpegVcM2ygylgXP_NCl9Vm10X4rsnYdZ1R90ZL6SBtMOaia8ennxgZMLGbb2-jBnr-TpIMTY2XmJUrQU27_6KiiuiJC7GzasJLxdOnlCumZIaRYXIiGxxIabSeC6-DEgcXZWZLek3ditkPTfHOGeffeUsvUdv5cX3lIm1LXtr6ZN2_Y7OGOMsqD52x94Z6mJUIQmYIPbPAUpSwaZJjQYjmt-pGBopwcuzeumOoyPxoofQ",
            "e": "AQAB",
            "kid": "TE7BetVKYIPPGIz1h-YYa",
            "x5t": "u6AjU8wPJNfNtKwaRc8HQ-YEdGE",
            "x5c": [
                "MIIDDTCCAfWgAwIBAgIJM80vcQ/lRJbeMA0GCSqGSIb3DQEBCwUAMCQxIjAgBgNVBAMTGWRldi1wMndzZnc2aC51cy5hdXRoMC5jb20wHhcNMjIwMTA2MDUwMDM4WhcNMzUwOTE1MDUwMDM4WjAkMSIwIAYDVQQDExlkZXYtcDJ3c2Z3NmgudXMuYXV0aDAuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAyFYEbpXjauy3jhQl2h7I/7nJE3ofmSA63TuqRBk/u+Ea446cFV8S+hMmHDzytEild3givHbnW3WfCmBX5iuoPi1BzrBBNba3rnrKB2ni2gpegVcM2ygylgXP/NCl9Vm10X4rsnYdZ1R90ZL6SBtMOaia8ennxgZMLGbb2+jBnr+TpIMTY2XmJUrQU27/6KiiuiJC7GzasJLxdOnlCumZIaRYXIiGxxIabSeC6+DEgcXZWZLek3ditkPTfHOGeffeUsvUdv5cX3lIm1LXtr6ZN2/Y7OGOMsqD52x94Z6mJUIQmYIPbPAUpSwaZJjQYjmt+pGBopwcuzeumOoyPxoofQIDAQABo0IwQDAPBgNVHRMBAf8EBTADAQH/MB0GA1UdDgQWBBRZt1zo3kvHajUYIIvVnwlTUnbR6DAOBgNVHQ8BAf8EBAMCAoQwDQYJKoZIhvcNAQELBQADggEBAAKbgAYrJEctR4viarBo1I/Kbd9CofbWu649p9C1VEb8jPcrHoirPRLn1z7m0lsgaODD8azuI4vtWwWiqw1C126Qdsi93fXreU3v6xWsUM+aamCHNMcsznPD3XdrbCVkrT0e7b+JSkwzmcXpNIjz4d6UUnsTHZoTqO0xevtmiXv63O7lHlFHYqOYGd2QoPF/Q2zt4gE/99DsqNwZF7LnxDJ+LTIAtHG0GES/plPVSkjdyzg/n6UVUDuZvkoUb/voXOJ23lOlmg6GamUmeoZopknFtkCBtpf8ZNueuwg1ab/6Kp7aCh1T8S2IsFnhUvRe4mhRFGaRRjeaafzQD608FPQ="
            ],
            "alg": "RS256"
        }
    ]
}
"#;

    

    let jwks_url = std::env::var("AUTH0_JWKS_URL").expect("AUTH0 credentials must be set");    
    let auth0 = Auth0::new(keys).expect("Error initializing auth0");

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
