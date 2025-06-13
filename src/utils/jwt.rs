use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub user_id: i32,
    pub role: String,
    pub exp: usize,
}

pub fn generate_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
        .map(|data| data.claims)
}
