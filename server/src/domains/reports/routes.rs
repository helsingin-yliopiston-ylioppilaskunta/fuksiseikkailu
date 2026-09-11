use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use super::{
    db,
    models::{
        CheckpointReport, CreateCheckpointReportPayload, CreateTeamReportPayload, TeamReport,
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
    path = "/reports/checkpoint",
    tag = "Reports",
    security(("bearer_auth" = [])),
    request_body = CreateCheckpointReportPayload,
    responses((status = 201, description = "Checkpoint report submitted", body = CheckpointReport))
)]
pub async fn create_checkpoint_report(
    State(state): State<AuthState>,
    _staff: RequireCheckpointStaff,
    Json(payload): Json<CreateCheckpointReportPayload>,
) -> Result<(StatusCode, Json<CheckpointReport>), AppError> {
    payload.validate()?;
    let report = db::create_checkpoint_report(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(report)))
}

#[utoipa::path(
    get,
    path = "/reports/checkpoint",
    tag = "Reports",
    security(("bearer_auth" = [])),
    responses((status = 200, description = "List all checkpoint reports", body = [CheckpointReport]))
)]
pub async fn list_checkpoint_reports(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
) -> Result<Json<Vec<CheckpointReport>>, AppError> {
    let reports = db::list_checkpoint_reports(&state.pool).await?;
    Ok(Json(reports))
}

#[utoipa::path(
    post,
    path = "/reports/team",
    tag = "Reports",
    security(("bearer_auth" = [])),
    request_body = CreateTeamReportPayload,
    responses((status = 201, description = "Team issue or disciplinary report submitted", body = TeamReport))
)]
pub async fn create_team_report(
    State(state): State<AuthState>,
    _staff: RequireCheckpointStaff,
    Json(payload): Json<CreateTeamReportPayload>,
) -> Result<(StatusCode, Json<TeamReport>), AppError> {
    payload.validate()?;
    let report = db::create_team_report(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(report)))
}

#[utoipa::path(
    get,
    path = "/reports/team",
    tag = "Reports",
    security(("bearer_auth" = [])),
    responses((status = 200, description = "List all team reports", body = [TeamReport]))
)]
pub async fn list_team_reports(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
) -> Result<Json<Vec<TeamReport>>, AppError> {
    let reports = db::list_team_reports(&state.pool).await?;
    Ok(Json(reports))
}
