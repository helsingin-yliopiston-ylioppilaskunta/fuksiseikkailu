use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use super::{
    db,
    models::{
        CheckpointRating, CreateCheckpointRatingPayload, CreateTeamRatingPayload, TeamRating,
    },
};
use crate::{
    domains::auth::{
        AuthState,
        extractor::{RequireAdmin, RequireCheckpointStaff},
    },
    errors::AppError,
};

#[utoipa::path(
    post,
    path = "/ratings/checkpoint",
    tag = "Ratings",
    request_body = CreateCheckpointRatingPayload,
    responses((status = 201, description = "Checkpoint rating submitted", body = CheckpointRating))
)]
pub async fn rate_checkpoint(
    State(state): State<AuthState>,
    Json(payload): Json<CreateCheckpointRatingPayload>,
) -> Result<(StatusCode, Json<CheckpointRating>), AppError> {
    payload.validate()?;
    let rating = db::rate_checkpoint(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(rating)))
}

#[utoipa::path(
    post,
    path = "/ratings/team",
    tag = "Ratings",
    security(("bearer_auth" = [])),
    request_body = CreateTeamRatingPayload,
    responses((status = 200, description = "Team spirit rating submitted or updated", body = TeamRating))
)]
pub async fn rate_team(
    State(state): State<AuthState>,
    staff: RequireCheckpointStaff,
    Json(payload): Json<CreateTeamRatingPayload>,
) -> Result<Json<TeamRating>, AppError> {
    payload.validate()?;

    if staff.0.role == crate::domains::users::models::Role::Checkpoint {
        if let Some(assigned_cp) = staff.0.checkpoint_id {
            if assigned_cp != payload.checkpoint_id {
                return Err(AppError::Forbidden(
                    "You can only rate teams from your assigned checkpoint".to_string(),
                ));
            }
        }
    }

    let rating = db::rate_team(&state.pool, &payload).await?;
    Ok(Json(rating))
}

#[utoipa::path(
    get,
    path = "/ratings/checkpoint/{checkpoint_id}",
    tag = "Ratings",
    security(("bearer_auth" = [])),
    params(("checkpoint_id" = Uuid, Path, description = "Checkpoint ID")),
    responses((status = 200, description = "Get checkpoint ratings", body = [CheckpointRating]))
)]
pub async fn get_checkpoint_ratings(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(checkpoint_id): Path<Uuid>,
) -> Result<Json<Vec<CheckpointRating>>, AppError> {
    let ratings = db::list_ratings_for_checkpoint(&state.pool, checkpoint_id).await?;
    Ok(Json(ratings))
}

#[utoipa::path(
    get,
    path = "/ratings/team/{team_id}",
    tag = "Ratings",
    security(("bearer_auth" = [])),
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses((status = 200, description = "Get team spirit ratings", body = [TeamRating]))
)]
pub async fn get_team_ratings(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(team_id): Path<Uuid>,
) -> Result<Json<Vec<TeamRating>>, AppError> {
    let ratings = db::list_ratings_for_team(&state.pool, team_id).await?;
    Ok(Json(ratings))
}
