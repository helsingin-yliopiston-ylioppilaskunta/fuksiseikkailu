use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    CheckpointRating, CreateCheckpointRatingPayload, CreateTeamRatingPayload, TeamRating,
};
use crate::errors::AppError;

pub async fn rate_checkpoint(
    pool: &PgPool,
    payload: &CreateCheckpointRatingPayload,
) -> Result<CheckpointRating, AppError> {
    let rating = sqlx::query_as!(
        CheckpointRating,
        r#"
        INSERT INTO checkpoint_ratings (checkpoint_id, team_id, rating)
        VALUES ($1, $2, $3)
        RETURNING id, checkpoint_id, team_id, rating, created_at, updated_at
        "#,
        payload.checkpoint_id,
        payload.team_id,
        payload.rating
    )
    .fetch_one(pool)
    .await?;

    Ok(rating)
}

pub async fn rate_team(
    pool: &PgPool,
    payload: &CreateTeamRatingPayload,
) -> Result<TeamRating, AppError> {
    let rating = sqlx::query_as!(
        TeamRating,
        r#"
        INSERT INTO team_ratings (team_id, checkpoint_id, rating)
        VALUES ($1, $2, $3)
        ON CONFLICT (checkpoint_id, team_id) DO UPDATE
        SET 
            rating = EXCLUDED.rating,
            updated_at = NOW(),
            deleted_at = NULL
        RETURNING id, team_id, checkpoint_id, rating, created_at, updated_at
        "#,
        payload.team_id,
        payload.checkpoint_id,
        payload.rating
    )
    .fetch_one(pool)
    .await?;

    Ok(rating)
}

pub async fn list_ratings_for_checkpoint(
    pool: &PgPool,
    checkpoint_id: Uuid,
) -> Result<Vec<CheckpointRating>, AppError> {
    let ratings = sqlx::query_as!(
        CheckpointRating,
        r#"
        SELECT id, checkpoint_id, team_id, rating, created_at, updated_at
        FROM checkpoint_ratings
        WHERE checkpoint_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        checkpoint_id
    )
    .fetch_all(pool)
    .await?;

    Ok(ratings)
}

pub async fn list_ratings_for_team(
    pool: &PgPool,
    team_id: Uuid,
) -> Result<Vec<TeamRating>, AppError> {
    let ratings = sqlx::query_as!(
        TeamRating,
        r#"
        SELECT id, team_id, checkpoint_id, rating, created_at, updated_at
        FROM team_ratings
        WHERE team_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        team_id
    )
    .fetch_all(pool)
    .await?;

    Ok(ratings)
}
