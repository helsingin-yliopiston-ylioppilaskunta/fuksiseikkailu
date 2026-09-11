use sqlx::PgPool;

use super::models::{
    CheckpointReport, CreateCheckpointReportPayload, CreateTeamReportPayload, TeamReport,
};
use crate::errors::AppError;

pub async fn create_checkpoint_report(
    pool: &PgPool,
    payload: &CreateCheckpointReportPayload,
) -> Result<CheckpointReport, AppError> {
    let report = sqlx::query_as!(
        CheckpointReport,
        r#"
        INSERT INTO checkpoint_reports (checkpoint_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, checkpoint_id, title, content, created_at, updated_at
        "#,
        payload.checkpoint_id,
        payload.title,
        payload.content
    )
    .fetch_one(pool)
    .await?;

    Ok(report)
}

pub async fn list_checkpoint_reports(pool: &PgPool) -> Result<Vec<CheckpointReport>, AppError> {
    let reports = sqlx::query_as!(
        CheckpointReport,
        r#"
        SELECT id, checkpoint_id, title, content, created_at, updated_at
        FROM checkpoint_reports
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(reports)
}

pub async fn create_team_report(
    pool: &PgPool,
    payload: &CreateTeamReportPayload,
) -> Result<TeamReport, AppError> {
    let report = sqlx::query_as!(
        TeamReport,
        r#"
        INSERT INTO team_reports (team_id, checkpoint_id, title, content)
        VALUES ($1, $2, $3, $4)
        RETURNING id, team_id, checkpoint_id, title, content, created_at, updated_at
        "#,
        payload.team_id,
        payload.checkpoint_id,
        payload.title,
        payload.content
    )
    .fetch_one(pool)
    .await?;

    Ok(report)
}

pub async fn list_team_reports(pool: &PgPool) -> Result<Vec<TeamReport>, AppError> {
    let reports = sqlx::query_as!(
        TeamReport,
        r#"
        SELECT id, team_id, checkpoint_id, title, content, created_at, updated_at
        FROM team_reports
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(reports)
}
