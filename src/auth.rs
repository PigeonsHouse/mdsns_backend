use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use log::debug;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}

pub async fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let jwk_url = std::env::var("JWK_URL").expect("JWK_URL must be set");
    let jwk_issuer = std::env::var("JWK_ISSUER").expect("JWK_ISSUER must be set");
    debug!("jwk_url:{}", jwk_url);
    debug!("jwk_issuer:[{}]", jwk_issuer);

    let jwks = fetch_jwks(jwk_url.as_str())
        .await
        .expect("failed to fetch jwks");

    // issとsub（user_id）をチェック
    // 必須ではなく、カスタマイズできる
    let validations = vec![Validation::Issuer(jwk_issuer), Validation::SubjectPresent];

    let kid = match token_kid(token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };

    debug!("kid:{:?}", kid);

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    debug!("jwk:{:?}", jwk);

    let res = validate(token, jwk, validations);
    debug!("res.is_ok():{}", res.is_ok());
    Ok(res.is_ok())
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    Ok(val)
}
