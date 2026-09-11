use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{domains::users::models::Role, errors::AppError};

/// Claims embedded inside every issued Access JWT
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // User ID
    pub role: Role,
    pub checkpoint_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub exp: usize, // Expiration Unix Timestamp
    pub iat: usize, // Issued At Unix Timestamp
}

/// Encodes a JWT token with user context and expiration TTL in seconds.
pub fn encode_jwt(
    user_id: Uuid,
    role: Role,
    checkpoint_id: Option<Uuid>,
    team_id: Option<Uuid>,
    secret: &str,
    expiration_seconds: u64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let expire = now + Duration::seconds(expiration_seconds as i64);

    let claims = Claims {
        sub: user_id,
        role,
        checkpoint_id,
        team_id,
        exp: expire.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(format!("JWT encoding error: {e}")))
}

/// Decodes and validates a JWT token signature and expiration.
pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized("Invalid or expired access token".to_string()))
}
