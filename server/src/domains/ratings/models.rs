use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CheckpointRating {
    pub id: Uuid,
    pub checkpoint_id: Uuid,
    pub team_id: Option<Uuid>,
    pub rating: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct TeamRating {
    pub id: Uuid,
    pub team_id: Uuid,
    pub checkpoint_id: Uuid,
    pub rating: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCheckpointRatingPayload {
    pub checkpoint_id: Uuid,
    pub team_id: Option<Uuid>,
    #[validate(range(min = 0, max = 5, message = "Rating must be between 0 and 5"))]
    pub rating: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTeamRatingPayload {
    pub team_id: Uuid,
    pub checkpoint_id: Uuid,
    #[validate(range(min = 0, max = 5, message = "Rating must be between 0 and 5"))]
    pub rating: i32,
}
