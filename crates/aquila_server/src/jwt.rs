use aquila_core::prelude::{AuthError, User};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub scopes: Vec<String>,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn mint(
        &self,
        subject: String,
        scopes: Vec<String>,
        duration_seconds: u64,
    ) -> Result<String, anyhow::Error> {
        let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + duration_seconds;
        let claims = Claims {
            sub: subject,
            exp: expiration as usize,
            scopes,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;
        Ok(token)
    }

    pub fn verify(&self, token: &str) -> Result<User, AuthError> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(User {
            id: token_data.claims.sub,
            scopes: token_data.claims.scopes,
        })
    }
}
