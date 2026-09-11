use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    Checkpoint, CheckpointCategory, CreateCheckpoint, PublicCheckpoint, UpdateCheckpoint,
};
use crate::errors::AppError;

pub async fn list_public_checkpoints(pool: &PgPool) -> Result<Vec<PublicCheckpoint>, AppError> {
    let checkpoints = sqlx::query_as!(
        PublicCheckpoint,
        r#"
        SELECT 
            id, area_id, number, name, category AS "category: CheckpointCategory", 
            location_name, latitude, longitude, accessible, lanes, 
            checkpoint_description, url, cancelled, created_at, updated_at
        FROM checkpoints
        WHERE deleted_at IS NULL
        ORDER BY number ASC NULLS LAST, name ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(checkpoints)
}

pub async fn list_all_checkpoints(pool: &PgPool) -> Result<Vec<Checkpoint>, AppError> {
    let checkpoints = sqlx::query_as!(
        Checkpoint,
        r#"
        SELECT 
            id, area_id, number, name, category AS "category: CheckpointCategory", 
            location_name, latitude, longitude, accessible, lanes, 
            checkpoint_description, org_description, requirements, execution, 
            url, contact_person, contact_email, contact_phone, cancelled, 
            created_at, updated_at
        FROM checkpoints
        WHERE deleted_at IS NULL
        ORDER BY number ASC NULLS LAST, name ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(checkpoints)
}

pub async fn get_checkpoint(pool: &PgPool, id: Uuid) -> Result<Checkpoint, AppError> {
    let checkpoint = sqlx::query_as!(
        Checkpoint,
        r#"
        SELECT 
            id, area_id, number, name, category AS "category: CheckpointCategory", 
            location_name, latitude, longitude, accessible, lanes, 
            checkpoint_description, org_description, requirements, execution, 
            url, contact_person, contact_email, contact_phone, cancelled, 
            created_at, updated_at
        FROM checkpoints
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(checkpoint)
}

pub async fn create_checkpoint(
    pool: &PgPool,
    payload: &CreateCheckpoint,
) -> Result<Checkpoint, AppError> {
    let category = payload.category.unwrap_or(CheckpointCategory::Other);
    let latitude = payload.latitude.unwrap_or(0.0);
    let longitude = payload.longitude.unwrap_or(0.0);
    let accessible = payload.accessible.unwrap_or(true);
    let lanes = payload.lanes.unwrap_or(1);

    let checkpoint = sqlx::query_as!(
        Checkpoint,
        r#"
        INSERT INTO checkpoints (
            area_id, number, name, category, location_name, latitude, longitude, 
            accessible, lanes, checkpoint_description, org_description, 
            requirements, execution, url, contact_person, contact_email, contact_phone
        )
        VALUES ($1, $2, $3, $4::checkpoint_category, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING 
            id, area_id, number, name, category AS "category: CheckpointCategory", 
            location_name, latitude, longitude, accessible, lanes, 
            checkpoint_description, org_description, requirements, execution, 
            url, contact_person, contact_email, contact_phone, cancelled, 
            created_at, updated_at
        "#,
        payload.area_id,
        payload.number,
        payload.name,
        category as CheckpointCategory,
        payload.location_name,
        latitude,
        longitude,
        accessible,
        lanes,
        payload.checkpoint_description,
        payload.org_description,
        payload.requirements,
        payload.execution,
        payload.url,
        payload.contact_person,
        payload.contact_email,
        payload.contact_phone
    )
    .fetch_one(pool)
    .await?;

    Ok(checkpoint)
}

pub async fn update_checkpoint(
    pool: &PgPool,
    id: Uuid,
    payload: &UpdateCheckpoint,
) -> Result<Checkpoint, AppError> {
    let checkpoint = sqlx::query_as!(
        Checkpoint,
        r#"
        UPDATE checkpoints
        SET 
            area_id = COALESCE($1, area_id),
            number = COALESCE($2, number),
            name = COALESCE($3, name),
            category = COALESCE($4::checkpoint_category, category),
            location_name = COALESCE($5, location_name),
            latitude = COALESCE($6, latitude),
            longitude = COALESCE($7, longitude),
            accessible = COALESCE($8, accessible),
            lanes = COALESCE($9, lanes),
            checkpoint_description = COALESCE($10, checkpoint_description),
            org_description = COALESCE($11, org_description),
            requirements = COALESCE($12, requirements),
            execution = COALESCE($13, execution),
            url = COALESCE($14, url),
            contact_person = COALESCE($15, contact_person),
            contact_email = COALESCE($16, contact_email),
            contact_phone = COALESCE($17, contact_phone),
            cancelled = COALESCE($18, cancelled),
            updated_at = NOW()
        WHERE id = $19 AND deleted_at IS NULL
        RETURNING 
            id, area_id, number, name, category AS "category: CheckpointCategory", 
            location_name, latitude, longitude, accessible, lanes, 
            checkpoint_description, org_description, requirements, execution, 
            url, contact_person, contact_email, contact_phone, cancelled, 
            created_at, updated_at
        "#,
        payload.area_id,
        payload.number,
        payload.name,
        payload.category as Option<CheckpointCategory>,
        payload.location_name,
        payload.latitude,
        payload.longitude,
        payload.accessible,
        payload.lanes,
        payload.checkpoint_description,
        payload.org_description,
        payload.requirements,
        payload.execution,
        payload.url,
        payload.contact_person,
        payload.contact_email,
        payload.contact_phone,
        payload.cancelled,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(checkpoint)
}

pub async fn delete_checkpoint(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
        UPDATE checkpoints
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

pub async fn batch_import_checkpoints(
    pool: &PgPool,
    items: &[CreateCheckpoint],
) -> Result<Vec<Checkpoint>, AppError> {
    let mut tx = pool.begin().await?;
    let mut imported = Vec::with_capacity(items.len());

    for payload in items {
        let category = payload.category.unwrap_or(CheckpointCategory::Other);
        let latitude = payload.latitude.unwrap_or(0.0);
        let longitude = payload.longitude.unwrap_or(0.0);
        let accessible = payload.accessible.unwrap_or(true);
        let lanes = payload.lanes.unwrap_or(1);

        let checkpoint = sqlx::query_as!(
            Checkpoint,
            r#"
            INSERT INTO checkpoints (
                area_id, number, name, category, location_name, latitude, longitude, 
                accessible, lanes, checkpoint_description, org_description, 
                requirements, execution, url, contact_person, contact_email, contact_phone
            )
            VALUES ($1, $2, $3, $4::checkpoint_category, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING 
                id, area_id, number, name, category AS "category: CheckpointCategory", 
                location_name, latitude, longitude, accessible, lanes, 
                checkpoint_description, org_description, requirements, execution, 
                url, contact_person, contact_email, contact_phone, cancelled, 
                created_at, updated_at
            "#,
            payload.area_id,
            payload.number,
            payload.name,
            category as CheckpointCategory,
            payload.location_name,
            latitude,
            longitude,
            accessible,
            lanes,
            payload.checkpoint_description,
            payload.org_description,
            payload.requirements,
            payload.execution,
            payload.url,
            payload.contact_person,
            payload.contact_email,
            payload.contact_phone
        )
        .fetch_one(&mut *tx)
        .await?;

        imported.push(checkpoint);
    }

    tx.commit().await?;
    Ok(imported)
}
