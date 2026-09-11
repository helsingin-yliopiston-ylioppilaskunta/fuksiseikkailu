use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use super::{
    db,
    models::{Area, CreateArea, UpdateArea},
};
use crate::{
    domains::auth::{AuthState, extractor::RequireAdmin},
    errors::AppError,
};

#[utoipa::path(
    get,
    path = "/areas",
    tag = "Areas",
    responses((status = 200, description = "List all active areas", body = [Area]))
)]
pub async fn list_areas(State(state): State<AuthState>) -> Result<Json<Vec<Area>>, AppError> {
    let areas = db::list_areas(&state.pool).await?;
    Ok(Json(areas))
}

#[utoipa::path(
    get,
    path = "/areas/{id}",
    tag = "Areas",
    params(("id" = Uuid, Path, description = "Area ID")),
    responses(
        (status = 200, description = "Area details", body = Area),
        (status = 404, description = "Area not found")
    )
)]
pub async fn get_area(
    State(state): State<AuthState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Area>, AppError> {
    let area = db::get_area(&state.pool, id).await?;
    Ok(Json(area))
}

#[utoipa::path(
    post,
    path = "/areas",
    tag = "Areas",
    security(("bearer_auth" = [])),
    request_body = CreateArea,
    responses(
        (status = 201, description = "Area created", body = Area),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_area(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Json(payload): Json<CreateArea>,
) -> Result<(StatusCode, Json<Area>), AppError> {
    payload.validate()?;
    let area = db::create_area(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(area)))
}

#[utoipa::path(
    patch,
    path = "/areas/{id}",
    tag = "Areas",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Area ID")),
    request_body = UpdateArea,
    responses(
        (status = 200, description = "Area updated", body = Area),
        (status = 404, description = "Area not found")
    )
)]
pub async fn update_area(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateArea>,
) -> Result<Json<Area>, AppError> {
    payload.validate()?;
    let area = db::update_area(&state.pool, id, &payload).await?;
    Ok(Json(area))
}

#[utoipa::path(
    delete,
    path = "/areas/{id}",
    tag = "Areas",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Area ID")),
    responses(
        (status = 204, description = "Area deleted"),
        (status = 404, description = "Area not found")
    )
)]
pub async fn delete_area(
    State(state): State<AuthState>,
    RequireAdmin(_admin): RequireAdmin,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    db::delete_area(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
