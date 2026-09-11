use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use super::{
    db,
    models::{CreateNewsPayload, NewsArticle, UpdateNewsPayload},
};
use crate::{
    domains::auth::{
        AuthState,
        extractor::{OptionalAuthUser, RequireAdmin},
    },
    errors::AppError,
};

#[utoipa::path(
    get,
    path = "/news",
    tag = "News",
    responses((status = 200, description = "List news articles", body = [NewsArticle]))
)]
pub async fn list_news(
    State(state): State<AuthState>,
    OptionalAuthUser(auth_user): OptionalAuthUser,
) -> Result<Json<Vec<NewsArticle>>, AppError> {
    if let Some(user) = auth_user {
        if user.role == crate::domains::users::models::Role::Admin {
            let all_articles = db::list_all_news(&state.pool).await?;
            return Ok(Json(all_articles));
        }
    }

    let published_articles = db::list_published_news(&state.pool).await?;
    Ok(Json(published_articles))
}

#[utoipa::path(
    get,
    path = "/news/{id}",
    tag = "News",
    params(("id" = Uuid, Path, description = "Article ID")),
    responses(
        (status = 200, description = "Get news article details", body = NewsArticle),
        (status = 404, description = "Article not found")
    )
)]
pub async fn get_news_article(
    State(state): State<AuthState>,
    Path(id): Path<Uuid>,
) -> Result<Json<NewsArticle>, AppError> {
    let article = db::get_news_article(&state.pool, id).await?;
    Ok(Json(article))
}

#[utoipa::path(
    post,
    path = "/news",
    tag = "News",
    security(("bearer_auth" = [])),
    request_body = CreateNewsPayload,
    responses((status = 201, description = "News article created", body = NewsArticle))
)]
pub async fn create_news_article(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Json(payload): Json<CreateNewsPayload>,
) -> Result<(StatusCode, Json<NewsArticle>), AppError> {
    payload.validate()?;
    let article = db::create_news_article(&state.pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(article)))
}

#[utoipa::path(
    patch,
    path = "/news/{id}",
    tag = "News",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Article ID")),
    request_body = UpdateNewsPayload,
    responses((status = 200, description = "News article updated", body = NewsArticle))
)]
pub async fn update_news_article(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNewsPayload>,
) -> Result<Json<NewsArticle>, AppError> {
    payload.validate()?;
    let article = db::update_news_article(&state.pool, id, &payload).await?;
    Ok(Json(article))
}

#[utoipa::path(
    delete,
    path = "/news/{id}",
    tag = "News",
    security(("bearer_auth" = [])),
    params(("id" = Uuid, Path, description = "Article ID")),
    responses((status = 204, description = "News article deleted"))
)]
pub async fn delete_news_article(
    State(state): State<AuthState>,
    _admin: RequireAdmin,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    db::delete_news_article(&state.pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
