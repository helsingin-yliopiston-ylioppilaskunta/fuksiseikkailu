use sqlx::PgPool;
use uuid::Uuid;

use super::models::{Area, CreateArea, UpdateArea};
use crate::errors::AppError;

pub async fn list_areas(pool: &PgPool) -> Result<Vec<Area>, AppError> {
    let areas = sqlx::query_as!(
        Area,
        r#"
        SELECT id, name, created_at, updated_at
        FROM areas
        WHERE deleted_at IS NULL
        ORDER BY name ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(areas)
}

pub async fn get_area(pool: &PgPool, id: Uuid) -> Result<Area, AppError> {
    let area = sqlx::query_as!(
        Area,
        r#"
        SELECT id, name, created_at, updated_at
        FROM areas
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(area)
}

pub async fn create_area(pool: &PgPool, payload: &CreateArea) -> Result<Area, AppError> {
    let area = sqlx::query_as!(
        Area,
        r#"
        INSERT INTO areas (name)
        VALUES ($1)
        RETURNING id, name, created_at, updated_at
        "#,
        payload.name
    )
    .fetch_one(pool)
    .await?;

    Ok(area)
}

pub async fn update_area(pool: &PgPool, id: Uuid, payload: &UpdateArea) -> Result<Area, AppError> {
    let area = sqlx::query_as!(
        Area,
        r#"
        UPDATE areas
        SET 
            name = COALESCE($1, name),
            updated_at = NOW()
        WHERE id = $2 AND deleted_at IS NULL
        RETURNING id, name, created_at, updated_at
        "#,
        payload.name,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(area)
}

pub async fn delete_area(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE areas
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
