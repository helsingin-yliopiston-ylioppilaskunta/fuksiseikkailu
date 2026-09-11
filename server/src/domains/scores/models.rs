use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Score {
    pub id: Uuid,
    pub team_id: Uuid,
    pub checkpoint_id: Uuid,
    pub recorded_by_user_id: Option<Uuid>,
    pub score: i32,
    pub participants_present: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubmitScorePayload {
    pub team_id: Uuid,
    pub checkpoint_id: Uuid,

    #[validate(range(min = 0, max = 12, message = "Score must be between 0 and 12"))]
    pub score: i32,

    #[validate(range(min = 0, message = "Participants present must be non-negative"))]
    pub participants_present: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateScorePayload {
    #[validate(range(min = 0, max = 12, message = "Score must be between 0 and 12"))]
    pub score: Option<i32>,

    #[validate(range(min = 0, message = "Participants present must be non-negative"))]
    pub participants_present: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TeamLeaderboardEntry {
    pub team_id: Uuid,
    pub team_name: String,
    pub team_number: Option<i32>,
    pub total_score: i64,
    pub checkpoints_visited: i64,
}
