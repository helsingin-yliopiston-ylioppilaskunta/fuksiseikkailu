use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::trim::{deserialize_trimmed_option_string, deserialize_trimmed_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "checkpoint_category", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CheckpointCategory {
    Subject,
    Nation,
    Hobby,
    Other,
    Hyy,
    Yliopisto,
}

/// Publicly exposed checkpoint payload (excludes sensitive private admin fields)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PublicCheckpoint {
    pub id: Uuid,
    pub area_id: Option<Uuid>,
    pub number: Option<i32>,
    pub name: String,
    pub category: CheckpointCategory,
    pub location_name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub accessible: bool,
    pub lanes: i32,
    pub checkpoint_description: Option<serde_json::Value>,
    pub url: Option<String>,
    pub cancelled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Admin checkpoint model (includes internal operational details)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Checkpoint {
    pub id: Uuid,
    pub area_id: Option<Uuid>,
    pub number: Option<i32>,
    pub name: String,
    pub category: CheckpointCategory,
    pub location_name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub accessible: bool,
    pub lanes: i32,
    pub checkpoint_description: Option<serde_json::Value>,
    pub org_description: Option<serde_json::Value>,
    pub requirements: Option<String>,
    pub execution: Option<String>,
    pub url: Option<String>,
    pub contact_person: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub cancelled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCheckpoint {
    pub area_id: Option<Uuid>,
    pub number: Option<i32>,

    #[serde(deserialize_with = "deserialize_trimmed_string")]
    #[validate(length(min = 1, message = "Checkpoint name cannot be empty"))]
    pub name: String,

    pub category: Option<CheckpointCategory>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub location_name: Option<String>,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accessible: Option<bool>,
    pub lanes: Option<i32>,

    pub checkpoint_description: Option<serde_json::Value>,
    pub org_description: Option<serde_json::Value>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub requirements: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub execution: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub url: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub contact_person: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub contact_email: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub contact_phone: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCheckpoint {
    pub area_id: Option<Uuid>,
    pub number: Option<i32>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub name: Option<String>,

    pub category: Option<CheckpointCategory>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub location_name: Option<String>,

    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accessible: Option<bool>,
    pub lanes: Option<i32>,

    pub checkpoint_description: Option<serde_json::Value>,
    pub org_description: Option<serde_json::Value>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub requirements: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub execution: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub url: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub contact_person: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub contact_email: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub contact_phone: Option<String>,

    pub cancelled: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchImportPayload {
    #[validate(nested)]
    pub checkpoints: Vec<CreateCheckpoint>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BatchImportResponse {
    pub imported_count: usize,
    pub checkpoints: Vec<Checkpoint>,
}
