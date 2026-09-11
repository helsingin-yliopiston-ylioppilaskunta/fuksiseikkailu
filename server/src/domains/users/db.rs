use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateUser, Role, UpdateUser, User};
use crate::errors::AppError;

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, role AS "role: Role", email, phone, name, checkpoint_id, team_id, created_at, updated_at
        FROM users
        WHERE deleted_at IS NULL
        ORDER BY email ASC NULLS LAST
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, role AS "role: Role", email, phone, name, checkpoint_id, team_id, created_at, updated_at
        FROM users
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(user)
}

pub async fn create_user(pool: &PgPool, payload: &CreateUser) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (role, email, phone, name, checkpoint_id, team_id)
        VALUES ($1::user_role, $2, $3, $4, $5, $6)
        RETURNING id, role AS "role: Role", email, phone, name, checkpoint_id, team_id, created_at, updated_at
        "#,
        payload.role as Role,
        payload.email.as_ref().map(|e| e.to_lowercase()),
        payload.phone,
        payload.name,
        payload.checkpoint_id,
        payload.team_id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user(pool: &PgPool, id: Uuid, payload: &UpdateUser) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            email = COALESCE($1, email),
            phone = COALESCE($2, phone),
            name = COALESCE($3, name),
            checkpoint_id = COALESCE($4, checkpoint_id),
            team_id = COALESCE($5, team_id),
            updated_at = NOW()
        WHERE id = $6 AND deleted_at IS NULL
        RETURNING id, role AS "role: Role", email, phone, name, checkpoint_id, team_id, created_at, updated_at
        "#,
        payload.email.as_ref().map(|e| e.to_lowercase()),
        payload.phone,
        payload.name,
        payload.checkpoint_id,
        payload.team_id,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE users
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
