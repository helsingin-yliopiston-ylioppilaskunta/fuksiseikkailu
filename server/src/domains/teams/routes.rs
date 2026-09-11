use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use super::{
    db,
    models::{BatchImportTeamsPayload, BatchImportTeamsResponse, CreateTeam, Team, UpdateTeam},
};
use crate::{
    domains::auth::{AuthState, extractor::RequireAdmin},
    errors::AppError,
};

#[utoipa::path(
    get,
    path = "/teams",
    tag = "Teams",
    responses((status = 200, description = "List all active teams", body = [Team]))
)]
pub async fn list_teams(State(state): State<AuthState>) -> Result<Json<Vec<Team>>, AppError> {
    let teams = db::list_teams(&state.pool).await?;
    Ok(Json(teams))
}

#[utoipa::path(
    get,
    path = "/teams/{id}",
    tag = "Teams",
    params(("id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Team details", body = Team),
        (status = 404, description = "Team not found")
    )
)]
pub async fn get_team(
    State(state): State<AuthState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Team>, AppError> {
    let team = db::get_team(&state.pool, id).await?;
    Ok(Json(team))
}

#[utoipa::path(
    post,
    path = "/teams",
    tag = "Teams",
    security(("bearer_auth" = [])),
    request_body = CreateTeam,
    responses(
        (status = 201, description = "Team created", body = Team),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Team number conflict")
    )
)]
pub async fn create_team(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Json(payload): Json<CreateTeam>,
) -> Result<(StatusCode, Json<Team>), AppError> {
    payload.validate()?;
    let team = db::create_team(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(team)))
}

#[utoipa::path(
    post,
    path = "/teams/batch",
    tag = "Teams",
    security(("bearer_auth" = [])),
    request_body = BatchImportTeamsPayload,
    responses(
        (status = 201, description = "Teams imported successfully", body = BatchImportTeamsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn batch_import(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Json(payload): Json<BatchImportTeamsPayload>,
) -> Result<(StatusCode, Json<BatchImportTeamsResponse>), AppError> {
    payload.validate()?;

    let imported = db::batch_import_teams(&state.pool, &payload.teams).await?;
    let count = imported.len();

    Ok((
        StatusCode::CREATED,
        Json(BatchImportTeamsResponse {
            imported_count: count,
            teams: imported,
        }),
    ))
}

#[utoipa::path(
    patch,
    path = "/teams/{id}",
    tag = "Teams",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Team ID")),
    request_body = UpdateTeam,
    responses(
        (status = 200, description = "Team updated", body = Team),
        (status = 404, description = "Team not found")
    )
)]
pub async fn update_team(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTeam>,
) -> Result<Json<Team>, AppError> {
    payload.validate()?;
    let team = db::update_team(&state.pool, id, &payload).await?;
    Ok(Json(team))
}

#[utoipa::path(
    delete,
    path = "/teams/{id}",
    tag = "Teams",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 204, description = "Team deleted"),
        (status = 404, description = "Team not found")
    )
)]
pub async fn delete_team(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    db::delete_team(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
