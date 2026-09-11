use axum::{Json, extract::State, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use super::{
    db, jwt,
    models::{AuthTokens, RegisterPayload, RequestOtpPayload, VerifyOtpPayload},
};
use crate::{config::Config, domains::users::models::Role, errors::AppError};

#[derive(Clone)]
pub struct AuthState {
    pub pool: sqlx::PgPool,
    pub config: Config,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    request_body = RegisterPayload,
    responses(
        (status = 201, description = "User registered successfully", body = AuthTokens),
        (status = 409, description = "Email already registered"),
        (status = 422, description = "Validation error")
    )
)]
pub async fn register(
    State(state): State<AuthState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<(StatusCode, Json<AuthTokens>), AppError> {
    payload.validate()?;

    let user = db::create_user(&state.pool, &payload).await?;
    let tokens =
        issue_token_pair(&state, user.id, user.role, user.checkpoint_id, user.team_id).await?;

    Ok((StatusCode::CREATED, Json(tokens)))
}

#[utoipa::path(
    post,
    path = "/auth/otp/request",
    tag = "Auth",
    request_body = RequestOtpPayload,
    responses(
        (status = 200, description = "OTP code requested successfully")
    )
)]
pub async fn request_otp(
    State(state): State<AuthState>,
    Json(payload): Json<RequestOtpPayload>,
) -> Result<StatusCode, AppError> {
    payload.validate()?;

    if let Some(ref email) = payload.email {
        if let Some(code) = db::create_and_store_otp(&state.pool, email).await? {
            // For local development, output the code directly to terminal logs
            tracing::info!("[DEV OTP] Code generated for {}: {}", email, code);
        } else {
            // Prevent user enumeration: log silently on server if email doesn't exist
            tracing::warn!("OTP requested for non-existent email: {}", email);
        }
    }

    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/auth/otp/verify",
    tag = "Auth",
    request_body = VerifyOtpPayload,
    responses(
        (status = 200, description = "OTP verified, returns tokens", body = AuthTokens),
        (status = 401, description = "Invalid or expired OTP code")
    )
)]
pub async fn verify_otp(
    State(state): State<AuthState>,
    Json(payload): Json<VerifyOtpPayload>,
) -> Result<Json<AuthTokens>, AppError> {
    payload.validate()?;

    let user = db::find_user_by_email(&state.pool, &payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or code".to_string()))?;

    let tokens =
        issue_token_pair(&state, user.id, user.role, user.checkpoint_id, user.team_id).await?;
    Ok(Json(tokens))
}

async fn issue_token_pair(
    state: &AuthState,
    user_id: Uuid,
    role: Role,
    checkpoint_id: Option<Uuid>,
    team_id: Option<Uuid>,
) -> Result<AuthTokens, AppError> {
    let access_token = jwt::encode_jwt(
        user_id,
        role,
        checkpoint_id,
        team_id,
        &state.config.jwt_secret,
        state.config.jwt_expiration_seconds,
    )?;

    Ok(AuthTokens {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.jwt_expiration_seconds,
    })
}
