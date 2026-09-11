use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::trim::{deserialize_trimmed_option_string, deserialize_trimmed_string};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub number: Option<i32>,
    pub participants: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTeam {
    #[serde(deserialize_with = "deserialize_trimmed_string")]
    #[validate(length(min = 1, message = "Team name cannot be empty"))]
    pub name: String,

    pub number: Option<i32>,

    #[validate(range(min = 0, message = "Participants count must be non-negative"))]
    pub participants: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTeam {
    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub name: Option<String>,

    pub number: Option<i32>,

    #[validate(range(min = 0, message = "Participants count must be non-negative"))]
    pub participants: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchImportTeamsPayload {
    #[validate(nested)]
    pub teams: Vec<CreateTeam>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BatchImportTeamsResponse {
    pub imported_count: usize,
    pub teams: Vec<Team>,
}
