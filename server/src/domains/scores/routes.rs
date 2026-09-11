use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use super::{
    db,
    models::{Score, SubmitScorePayload, TeamLeaderboardEntry, UpdateScorePayload},
};
use crate::{
    domains::auth::{
        AuthState,
        extractor::{RequireAdmin, RequireCheckpointStaff},
    },
    errors::AppError,
};

#[utoipa::path(
    get,
    path = "/scores/leaderboard",
    tag = "Scores",
    responses((status = 200, description = "Get event leaderboard", body = [TeamLeaderboardEntry]))
)]
pub async fn get_leaderboard(
    State(state): State<AuthState>,
) -> Result<Json<Vec<TeamLeaderboardEntry>>, AppError> {
    let leaderboard = db::get_leaderboard(&state.pool).await?;
    Ok(Json(leaderboard))
}

#[utoipa::path(
    post,
    path = "/scores",
    tag = "Scores",
    security(("bearer_auth" = [])),
    request_body = SubmitScorePayload,
    responses(
        (status = 200, description = "Score submitted or updated", body = Score),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Checkpoint staff or Admin required")
    )
)]
pub async fn submit_score(
    State(state): State<AuthState>,
    staff: RequireCheckpointStaff,
    Json(payload): Json<SubmitScorePayload>,
) -> Result<(StatusCode, Json<Score>), AppError> {
    payload.validate()?;

    // Checkpoint staff can only record scores for their assigned checkpoint
    if staff.0.role == crate::domains::users::models::Role::Checkpoint {
        if let Some(assigned_cp) = staff.0.checkpoint_id {
            if assigned_cp != payload.checkpoint_id {
                return Err(AppError::Forbidden(
                    "You can only submit scores for your assigned checkpoint".to_string(),
                ));
            }
        } else {
            return Err(AppError::Forbidden(
                "No checkpoint assigned to your user account".to_string(),
            ));
        }
    }

    let score = db::submit_or_update_score(&state.pool, staff.0.id, &payload).await?;
    Ok((StatusCode::OK, Json(score)))
}

#[utoipa::path(
    get,
    path = "/scores/checkpoint/{checkpoint_id}",
    tag = "Scores",
    security(("bearer_auth" = [])),
    params(("checkpoint_id" = Uuid, Path, description = "Checkpoint ID")),
    responses((status = 200, description = "List scores for a checkpoint", body = [Score]))
)]
pub async fn list_by_checkpoint(
    State(state): State<AuthState>,
    _staff: RequireCheckpointStaff,
    Path(checkpoint_id): Path<Uuid>,
) -> Result<Json<Vec<Score>>, AppError> {
    let scores = db::list_scores_by_checkpoint(&state.pool, checkpoint_id).await?;
    Ok(Json(scores))
}

#[utoipa::path(
    get,
    path = "/scores/team/{team_id}",
    tag = "Scores",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses((status = 200, description = "List scores for a team", body = [Score]))
)]
pub async fn list_by_team(
    State(state): State<AuthState>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<Vec<Score>>, AppError> {
    let scores = db::list_scores_by_team(&state.pool, team_id).await?;
    Ok(Json(scores))
}

#[utoipa::path(
    patch,
    path = "/scores/{id}",
    tag = "Scores",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Score ID")),
    request_body = UpdateScorePayload,
    responses((status = 200, description = "Score updated", body = Score))
)]
pub async fn update_score(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateScorePayload>,
) -> Result<Json<Score>, AppError> {
    payload.validate()?;
    let score = db::update_score(&state.pool, id, &payload).await?;
    Ok(Json(score))
}
