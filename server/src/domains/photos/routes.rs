use axum::{
    Json,
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode, header},
};
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use uuid::Uuid;
use validator::Validate;

use super::{
    db,
    models::{
        CreatePhotoPayload, Photo, PhotoTeamSuggestion, SubmitSuggestionPayload, UpdatePhotoPayload,
    },
};
use crate::{
    domains::auth::{
        AuthState,
        extractor::{OptionalAuthUser, RequireAdmin},
    },
    errors::AppError,
};

/// Computes a SHA-256 fingerprint hash of request IP + User-Agent header
fn compute_voter_hash(headers: &HeaderMap, addr: Option<SocketAddr>) -> String {
    let ip = addr.map(|a| a.ip().to_string()).unwrap_or_default();
    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let mut hasher = Sha256::new();
    hasher.update(format!("{ip}:{ua}:fuksiseikkailu-salt").as_bytes());
    hex::encode(hasher.finalize())
}

#[utoipa::path(
    get,
    path = "/photos",
    tag = "Photos",
    responses((status = 200, description = "List photos", body = [Photo]))
)]
pub async fn list_photos(
    State(state): State<AuthState>,
    OptionalAuthUser(auth_user): OptionalAuthUser,
) -> Result<Json<Vec<Photo>>, AppError> {
    if let Some(user) = auth_user {
        if user.role == crate::domains::users::models::Role::Admin {
            let photos = db::list_all_photos(&state.pool).await?;
            return Ok(Json(photos));
        }
    }

    let public_photos = db::list_published_photos(&state.pool).await?;
    Ok(Json(public_photos))
}

#[utoipa::path(
    post,
    path = "/photos",
    tag = "Photos",
    security(("bearer_auth" = [])),
    request_body = CreatePhotoPayload,
    responses((status = 201, description = "Photo metadata created", body = Photo))
)]
pub async fn create_photo(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Json(payload): Json<CreatePhotoPayload>,
) -> Result<(StatusCode, Json<Photo>), AppError> {
    payload.validate()?;
    let photo = db::create_photo(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(photo)))
}

#[utoipa::path(
    patch,
    path = "/photos/{id}",
    tag = "Photos",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Photo ID")),
    request_body = UpdatePhotoPayload,
    responses((status = 200, description = "Photo metadata updated", body = Photo))
)]
pub async fn update_photo(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePhotoPayload>,
) -> Result<Json<Photo>, AppError> {
    payload.validate()?;
    let photo = db::update_photo(&state.pool, id, &payload).await?;
    Ok(Json(photo))
}

#[utoipa::path(
    delete,
    path = "/photos/{id}",
    tag = "Photos",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Photo ID")),
    responses((status = 204, description = "Photo deleted"))
)]
pub async fn delete_photo(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    db::delete_photo(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/photos/{id}/vote",
    tag = "Photos",
    params(("id" = Uuid, Path, description = "Photo ID")),
    responses(
        (status = 200, description = "Vote recorded"),
        (status = 409, description = "Already voted for this photo")
    )
)]
pub async fn vote_photo(
    State(state): State<AuthState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let voter_hash = compute_voter_hash(&headers, Some(addr));
    db::cast_vote(&state.pool, id, &voter_hash).await?;
    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/photos/{id}/suggest",
    tag = "Photos",
    params(("id" = Uuid, Path, description = "Photo ID")),
    request_body = SubmitSuggestionPayload,
    responses((status = 201, description = "Team tag suggestion submitted", body = PhotoTeamSuggestion))
)]
pub async fn suggest_team(
    State(state): State<AuthState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(payload): Json<SubmitSuggestionPayload>,
) -> Result<(StatusCode, Json<PhotoTeamSuggestion>), AppError> {
    payload.validate()?;
    let voter_hash = compute_voter_hash(&headers, Some(addr));

    let suggestion = db::submit_suggestion(
        &state.pool,
        id,
        payload.suggested_team_number,
        Some(&voter_hash),
    )
    .await?;

    Ok((StatusCode::CREATED, Json(suggestion)))
}

#[utoipa::path(
    get,
    path = "/photos/{id}/suggestions",
    tag = "Photos",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Photo ID")),
    responses((status = 200, description = "List team tag suggestions for photo", body = [PhotoTeamSuggestion]))
)]
pub async fn list_suggestions(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<PhotoTeamSuggestion>>, AppError> {
    let suggestions = db::list_suggestions(&state.pool, id).await?;
    Ok(Json(suggestions))
}
