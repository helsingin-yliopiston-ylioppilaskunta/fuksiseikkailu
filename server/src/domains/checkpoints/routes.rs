use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use super::{
    db,
    models::{Checkpoint, CreateCheckpoint, PublicCheckpoint, UpdateCheckpoint},
};
use crate::{
    domains::{
        auth::{
            AuthState,
            extractor::{OptionalAuthUser, RequireAdmin, RequireCheckpointStaff},
        },
        checkpoints::models::{BatchImportPayload, BatchImportResponse},
    },
    errors::AppError,
};

#[utoipa::path(
    get,
    path = "/checkpoints",
    tag = "Checkpoints",
    responses((status = 200, description = "List checkpoints (sanitized public payload for guests)", body = [PublicCheckpoint]))
)]
pub async fn list_checkpoints(
    State(state): State<AuthState>,
    OptionalAuthUser(auth_user): OptionalAuthUser,
) -> Result<ResponsePayload, AppError> {
    // If authenticated as Admin/Staff, return complete operational records
    if let Some(user) = auth_user {
        if user.role == crate::domains::users::models::Role::Admin
            || user.role == crate::domains::users::models::Role::Checkpoint
        {
            let checkpoints = db::list_all_checkpoints(&state.pool).await?;
            return Ok(ResponsePayload::Admin(checkpoints));
        }
    }

    let public_checkpoints = db::list_public_checkpoints(&state.pool).await?;
    Ok(ResponsePayload::Public(public_checkpoints))
}

pub enum ResponsePayload {
    Public(Vec<PublicCheckpoint>),
    Admin(Vec<Checkpoint>),
}

impl axum::response::IntoResponse for ResponsePayload {
    fn into_response(self) -> axum::response::Response {
        match self {
            ResponsePayload::Public(data) => Json(data).into_response(),
            ResponsePayload::Admin(data) => Json(data).into_response(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/checkpoints/{id}",
    tag = "Checkpoints",
    params(("id" = Uuid, Path, description = "Checkpoint ID")),
    responses(
        (status = 200, description = "Checkpoint details", body = Checkpoint),
        (status = 404, description = "Checkpoint not found")
    )
)]
pub async fn get_checkpoint(
    State(state): State<AuthState>,
    _staff: RequireCheckpointStaff,
    Path(id): Path<Uuid>,
) -> Result<Json<Checkpoint>, AppError> {
    let checkpoint = db::get_checkpoint(&state.pool, id).await?;
    Ok(Json(checkpoint))
}

#[utoipa::path(
    post,
    path = "/checkpoints",
    tag = "Checkpoints",
    security(("bearer_auth" = [])),
    request_body = CreateCheckpoint,
    responses(
        (status = 201, description = "Checkpoint created", body = Checkpoint),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_checkpoint(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Json(payload): Json<CreateCheckpoint>,
) -> Result<(StatusCode, Json<Checkpoint>), AppError> {
    payload.validate()?;
    let checkpoint = db::create_checkpoint(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(checkpoint)))
}

#[utoipa::path(
    patch,
    path = "/checkpoints/{id}",
    tag = "Checkpoints",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Checkpoint ID")),
    request_body = UpdateCheckpoint,
    responses(
        (status = 200, description = "Checkpoint updated", body = Checkpoint),
        (status = 404, description = "Checkpoint not found")
    )
)]
pub async fn update_checkpoint(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCheckpoint>,
) -> Result<Json<Checkpoint>, AppError> {
    payload.validate()?;
    let checkpoint = db::update_checkpoint(&state.pool, id, &payload).await?;
    Ok(Json(checkpoint))
}

#[utoipa::path(
    delete,
    path = "/checkpoints/{id}",
    tag = "Checkpoints",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Checkpoint ID")),
    responses(
        (status = 204, description = "Checkpoint deleted"),
        (status = 404, description = "Checkpoint not found")
    )
)]
pub async fn delete_checkpoint(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    db::delete_checkpoint(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/checkpoints/batch",
    tag = "Checkpoints",
    security(("bearer_auth" = [])),
    request_body = BatchImportPayload,
    responses(
        (status = 201, description = "Checkpoints imported successfully", body = BatchImportResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin access required")
    )
)]
pub async fn batch_import(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Json(payload): Json<BatchImportPayload>,
) -> Result<(StatusCode, Json<BatchImportResponse>), AppError> {
    payload.validate()?;

    let imported = db::batch_import_checkpoints(&state.pool, &payload.checkpoints).await?;
    let count = imported.len();

    Ok((
        StatusCode::CREATED,
        Json(BatchImportResponse {
            imported_count: count,
            checkpoints: imported,
        }),
    ))
}
