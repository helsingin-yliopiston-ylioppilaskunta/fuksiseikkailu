use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use super::{AuthState, jwt};
use crate::{domains::users::models::Role, errors::AppError};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: Role,
    pub checkpoint_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_state = AuthState::from_ref(state);

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            AppError::Unauthorized("Invalid Authorization header format".to_string())
        })?;

        let claims = jwt::decode_jwt(token, &auth_state.config.jwt_secret)?;

        Ok(AuthUser {
            id: claims.sub,
            role: claims.role,
            checkpoint_id: claims.checkpoint_id,
            team_id: claims.team_id,
        })
    }
}

/// Requires the user to have Admin privileges
#[derive(Debug, Clone)]
pub struct RequireAdmin(pub AuthUser);

impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        if user.role != Role::Admin {
            return Err(AppError::Forbidden("Admin privileges required".to_string()));
        }
        Ok(RequireAdmin(user))
    }
}

/// Requires the user to be a Checkpoint Organizer or Admin
#[derive(Debug, Clone)]
pub struct RequireCheckpointStaff(pub AuthUser);

impl<S> FromRequestParts<S> for RequireCheckpointStaff
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        if user.role != Role::Admin && user.role != Role::Checkpoint {
            return Err(AppError::Forbidden(
                "Checkpoint staff privileges required".to_string(),
            ));
        }
        Ok(RequireCheckpointStaff(user))
    }
}

/// Optional extractor: returns `Some(AuthUser)` if a valid Bearer token is present,
/// or `None` if unauthenticated.
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
    AuthState: FromRef<S>,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthUser(Some(user))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
