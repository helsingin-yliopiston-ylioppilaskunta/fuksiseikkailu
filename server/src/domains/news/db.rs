use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateNewsPayload, NewsArticle, UpdateNewsPayload};
use crate::errors::AppError;

pub async fn list_published_news(pool: &PgPool) -> Result<Vec<NewsArticle>, AppError> {
    let articles = sqlx::query_as!(
        NewsArticle,
        r#"
        SELECT id, title, content, published_at, notification_sent_at, created_at, updated_at
        FROM news
        WHERE published_at IS NOT NULL AND published_at <= NOW() AND deleted_at IS NULL
        ORDER BY published_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn list_all_news(pool: &PgPool) -> Result<Vec<NewsArticle>, AppError> {
    let articles = sqlx::query_as!(
        NewsArticle,
        r#"
        SELECT id, title, content, published_at, notification_sent_at, created_at, updated_at
        FROM news
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn get_news_article(pool: &PgPool, id: Uuid) -> Result<NewsArticle, AppError> {
    let article = sqlx::query_as!(
        NewsArticle,
        r#"
        SELECT id, title, content, published_at, notification_sent_at, created_at, updated_at
        FROM news
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(article)
}

pub async fn create_news_article(
    pool: &PgPool,
    payload: &CreateNewsPayload,
) -> Result<NewsArticle, AppError> {
    let article = sqlx::query_as!(
        NewsArticle,
        r#"
        INSERT INTO news (title, content, published_at)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, published_at, notification_sent_at, created_at, updated_at
        "#,
        payload.title,
        payload.content,
        payload.published_at
    )
    .fetch_one(pool)
    .await?;

    Ok(article)
}

pub async fn update_news_article(
    pool: &PgPool,
    id: Uuid,
    payload: &UpdateNewsPayload,
) -> Result<NewsArticle, AppError> {
    let article = sqlx::query_as!(
        NewsArticle,
        r#"
        UPDATE news
        SET 
            title = COALESCE($1, title),
            content = COALESCE($2, content),
            published_at = COALESCE($3, published_at),
            updated_at = NOW()
        WHERE id = $4 AND deleted_at IS NULL
        RETURNING id, title, content, published_at, notification_sent_at, created_at, updated_at
        "#,
        payload.title,
        payload.content,
        payload.published_at,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(article)
}

pub async fn delete_news_article(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE news
        SET deleted_at = NOW()
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
