use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, get_current_timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // username
    pub exp: usize,
}

pub fn create_token(username: &str, secret: &str) -> String {
    let expiration = get_current_timestamp() + 3600; // 1 hour

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Token creation failed")
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, String> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}
