use sqlx::PgPool;
use uuid::Uuid;

use super::models::{Score, SubmitScorePayload, TeamLeaderboardEntry, UpdateScorePayload};
use crate::errors::AppError;

pub async fn submit_or_update_score(
    pool: &PgPool,
    user_id: Uuid,
    payload: &SubmitScorePayload,
) -> Result<Score, AppError> {
    let participants = payload.participants_present.unwrap_or(0);

    // Upsert pattern on unique_team_checkpoint_score constraint
    let score = sqlx::query_as!(
        Score,
        r#"
        INSERT INTO scores (team_id, checkpoint_id, recorded_by_user_id, score, participants_present)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (team_id, checkpoint_id) DO UPDATE
        SET 
            score = EXCLUDED.score,
            participants_present = EXCLUDED.participants_present,
            recorded_by_user_id = EXCLUDED.recorded_by_user_id,
            updated_at = NOW(),
            deleted_at = NULL
        RETURNING id, team_id, checkpoint_id, recorded_by_user_id, score, participants_present, created_at, updated_at
        "#,
        payload.team_id,
        payload.checkpoint_id,
        user_id,
        payload.score,
        participants
    )
    .fetch_one(pool)
    .await?;

    Ok(score)
}

pub async fn list_scores_by_checkpoint(
    pool: &PgPool,
    checkpoint_id: Uuid,
) -> Result<Vec<Score>, AppError> {
    let scores = sqlx::query_as!(
        Score,
        r#"
        SELECT id, team_id, checkpoint_id, recorded_by_user_id, score, participants_present, created_at, updated_at
        FROM scores
        WHERE checkpoint_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        checkpoint_id
    )
    .fetch_all(pool)
    .await?;

    Ok(scores)
}

pub async fn list_scores_by_team(pool: &PgPool, team_id: Uuid) -> Result<Vec<Score>, AppError> {
    let scores = sqlx::query_as!(
        Score,
        r#"
        SELECT id, team_id, checkpoint_id, recorded_by_user_id, score, participants_present, created_at, updated_at
        FROM scores
        WHERE team_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        team_id
    )
    .fetch_all(pool)
    .await?;

    Ok(scores)
}

pub async fn update_score(
    pool: &PgPool,
    id: Uuid,
    payload: &UpdateScorePayload,
) -> Result<Score, AppError> {
    let score = sqlx::query_as!(
        Score,
        r#"
        UPDATE scores
        SET 
            score = COALESCE($1, score),
            participants_present = COALESCE($2, participants_present),
            updated_at = NOW()
        WHERE id = $3 AND deleted_at IS NULL
        RETURNING id, team_id, checkpoint_id, recorded_by_user_id, score, participants_present, created_at, updated_at
        "#,
        payload.score,
        payload.participants_present,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(score)
}

pub async fn get_leaderboard(pool: &PgPool) -> Result<Vec<TeamLeaderboardEntry>, AppError> {
    let leaderboard = sqlx::query_as!(
        TeamLeaderboardEntry,
        r#"
        SELECT 
            t.id as "team_id!",
            t.name as "team_name!",
            t.number as "team_number",
            COALESCE(SUM(s.score), 0)::BIGINT as "total_score!",
            COUNT(s.id)::BIGINT as "checkpoints_visited!"
        FROM teams t
        LEFT JOIN scores s ON s.team_id = t.id AND s.deleted_at IS NULL
        WHERE t.deleted_at IS NULL
        GROUP BY t.id, t.name, t.number
        ORDER BY COALESCE(SUM(s.score), 0) DESC, COUNT(s.id) DESC, t.name ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(leaderboard)
}
