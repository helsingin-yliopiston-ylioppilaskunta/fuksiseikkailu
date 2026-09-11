use chrono::{Duration, Utc};
use rand::RngExt;
use sqlx::PgPool;
use uuid::Uuid;

use super::models::RegisterPayload;
use crate::{domains::users::models::Role, errors::AppError};

/// Generates a random 6-digit code, hashes it, saves it to the database,
/// and returns the unhashed raw code for logging/email dispatch.
pub async fn create_and_store_otp(pool: &PgPool, email: &str) -> Result<Option<String>, AppError> {
    // Rand 0.10+ syntax: rand::rng() and .random_range()
    let raw_code: String = format!("{:06}", rand::rng().random_range(0..1_000_000));

    // Hash the OTP using argon2 for secure DB storage
    let code_hash = crate::domains::auth::password::hash_password(&raw_code)?;
    let expires_at = Utc::now() + Duration::minutes(10);

    let result = sqlx::query!(
        r#"
        UPDATE users
        SET 
            otp_code_hash = $1,
            otp_expires_at = $2,
            otp_sent_at = NOW(),
            otp_attempts = 0
        WHERE LOWER(email) = LOWER($3) AND deleted_at IS NULL
        "#,
        code_hash,
        expires_at,
        email
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    Ok(Some(raw_code))
}

pub struct UserAuthInfo {
    pub id: Uuid,
    pub email: Option<String>,
    pub role: Role,
    pub checkpoint_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}

pub async fn create_user(
    pool: &PgPool,
    payload: &RegisterPayload,
) -> Result<UserAuthInfo, AppError> {
    let user = sqlx::query_as!(
        UserAuthInfo,
        r#"
        INSERT INTO users (email, role, checkpoint_id, team_id)
        VALUES (LOWER($1), $2::user_role, $3, $4)
        RETURNING id, email, role AS "role: Role", checkpoint_id, team_id
        "#,
        payload.email.as_deref(),
        payload.role as Role,
        payload.checkpoint_id,
        payload.team_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<UserAuthInfo>, AppError> {
    let user = sqlx::query_as!(
        UserAuthInfo,
        r#"
        SELECT id, email, role AS "role: Role", checkpoint_id, team_id
        FROM users
        WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<UserAuthInfo>, AppError> {
    let user = sqlx::query_as!(
        UserAuthInfo,
        r#"
        SELECT id, email, role AS "role: Role", checkpoint_id, team_id
        FROM users
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
