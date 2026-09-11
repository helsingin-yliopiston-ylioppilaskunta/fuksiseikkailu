use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateTeam, Team, UpdateTeam};
use crate::errors::AppError;

pub async fn list_teams(pool: &PgPool) -> Result<Vec<Team>, AppError> {
    let teams = sqlx::query_as!(
        Team,
        r#"
        SELECT id, name, number, participants, created_at, updated_at
        FROM teams
        WHERE deleted_at IS NULL
        ORDER BY number ASC NULLS LAST, name ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(teams)
}

pub async fn get_team(pool: &PgPool, id: Uuid) -> Result<Team, AppError> {
    let team = sqlx::query_as!(
        Team,
        r#"
        SELECT id, name, number, participants, created_at, updated_at
        FROM teams
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(team)
}

pub async fn create_team(pool: &PgPool, payload: &CreateTeam) -> Result<Team, AppError> {
    let participants = payload.participants.unwrap_or(0);

    let team = sqlx::query_as!(
        Team,
        r#"
        INSERT INTO teams (name, number, participants)
        VALUES ($1, $2, $3)
        RETURNING id, name, number, participants, created_at, updated_at
        "#,
        payload.name,
        payload.number,
        participants
    )
    .fetch_one(pool)
    .await?;

    Ok(team)
}

pub async fn batch_import_teams(
    pool: &PgPool,
    items: &[CreateTeam],
) -> Result<Vec<Team>, AppError> {
    let mut tx = pool.begin().await?;
    let mut imported = Vec::with_capacity(items.len());

    for payload in items {
        let participants = payload.participants.unwrap_or(0);

        let team = sqlx::query_as!(
            Team,
            r#"
            INSERT INTO teams (name, number, participants)
            VALUES ($1, $2, $3)
            RETURNING id, name, number, participants, created_at, updated_at
            "#,
            payload.name,
            payload.number,
            participants
        )
        .fetch_one(&mut *tx)
        .await?;

        imported.push(team);
    }

    tx.commit().await?;
    Ok(imported)
}

pub async fn update_team(pool: &PgPool, id: Uuid, payload: &UpdateTeam) -> Result<Team, AppError> {
    let team = sqlx::query_as!(
        Team,
        r#"
        UPDATE teams
        SET 
            name = COALESCE($1, name),
            number = COALESCE($2, number),
            participants = COALESCE($3, participants),
            updated_at = NOW()
        WHERE id = $4 AND deleted_at IS NULL
        RETURNING id, name, number, participants, created_at, updated_at
        "#,
        payload.name,
        payload.number,
        payload.participants,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(team)
}

pub async fn delete_team(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE teams
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
