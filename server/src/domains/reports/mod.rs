pub mod db;
pub mod models;
pub mod routes;

use axum::{Router, routing::get};

use crate::domains::auth::AuthState;

pub fn router(state: AuthState) -> Router {
    Router::new()
        .route(
            "/checkpoint",
            get(routes::list_checkpoint_reports).post(routes::create_checkpoint_report),
        )
        .route(
            "/team",
            get(routes::list_team_reports).post(routes::create_team_report),
        )
        .with_state(state)
}
