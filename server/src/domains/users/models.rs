use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::trim::deserialize_trimmed_option_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Checkpoint,
    Team,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub role: Role,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub checkpoint_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUser {
    pub role: Role,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    #[validate(email(message = "Invalid email address format"))]
    pub email: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub phone: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub name: Option<String>,

    pub checkpoint_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUser {
    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    #[validate(email(message = "Invalid email address format"))]
    pub email: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub phone: Option<String>,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub name: Option<String>,

    pub checkpoint_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}
