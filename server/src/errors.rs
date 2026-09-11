use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Resource not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Database error")]
    Database(sqlx::Error),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(ref db_err) => match db_err.code().as_deref() {
                Some("23505") => {
                    let message = match db_err.constraint() {
                        Some("users_email_key") => "A user with this email already exists.",
                        Some("teams_number_key") => "A team with this number already exists.",
                        Some("unique_team_checkpoint_score") => {
                            "A score has already been recorded for this team at this checkpoint."
                        }
                        Some("unique_photo_voter") => "You have already voted for this photo.",
                        Some("unique_checkpoint_team_rating") => {
                            "Rating already submitted for this team."
                        }
                        _ => "A resource with this unique constraint already exists.",
                    };
                    AppError::Conflict(message.to_string())
                }

                Some("23503") => {
                    let message = match db_err.constraint() {
                        Some("checkpoints_area_id_fkey") => "The specified area does not exist.",
                        Some("users_checkpoint_id_fkey") => {
                            "The specified checkpoint does not exist."
                        }
                        Some("users_team_id_fkey") => "The specified team does not exist.",
                        Some("scores_team_id_fkey") => "The specified team does not exist.",
                        Some("scores_checkpoint_id_fkey") => {
                            "The specified checkpoint does not exist."
                        }
                        _ => "Referenced entity does not exist.",
                    };
                    AppError::BadRequest(message.to_string())
                }

                _ => AppError::Database(err),
            },
            _ => AppError::Database(err),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(ref msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Forbidden(ref msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Conflict(ref msg) => (StatusCode::CONFLICT, msg.clone()),

            AppError::ValidationError(ref errs) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Invalid request body: {errs}"),
            ),

            AppError::Database(ref err) => {
                tracing::error!("Unhandled Database Error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal database error occurred.".to_string(),
                )
            }

            AppError::InternalServerError(ref err) => {
                tracing::error!("Unhandled Internal Server Error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal error occurred.".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
