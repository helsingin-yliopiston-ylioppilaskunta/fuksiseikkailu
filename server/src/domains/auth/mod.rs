pub mod db;
pub mod extractor;
pub mod jwt;
pub mod models;
pub mod password;
pub mod routes;

use axum::{Router, routing::post};

pub use extractor::{AuthUser, RequireAdmin, RequireCheckpointStaff};
pub use routes::AuthState;

pub fn router(state: AuthState) -> Router {
    Router::new()
        .route("/register", post(routes::register))
        .route("/otp/request", post(routes::request_otp))
        .route("/otp/verify", post(routes::verify_otp))
        .with_state(state)
}
